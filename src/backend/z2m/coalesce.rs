use std::collections::HashMap;
use std::time::Duration;
use tokio::time::Instant;

use hue::api::{GroupedLightUpdate, LightUpdate, ResourceLink};

#[derive(Debug, Clone)]
pub enum PendingUpdate {
    Light(LightUpdate),
    GroupedLight(GroupedLightUpdate),
}

#[derive(Debug)]
struct CoalesceEntry {
    last_sent: Instant,
    pending: Option<PendingUpdate>,
    deadline: Option<Instant>,
    in_flight: bool,
    in_flight_since: Option<Instant>,
    coalesced_count: u32,
    rolling_latency: Option<Duration>,
}

impl CoalesceEntry {
    fn effective_interval(&self, min_interval: Duration) -> Duration {
        const MAX_ADAPTIVE_INTERVAL: Duration = Duration::from_millis(1000);
        let interval = match self.rolling_latency {
            Some(lat) => min_interval.max(lat),
            None => min_interval,
        };
        interval.min(MAX_ADAPTIVE_INTERVAL)
    }

    fn check_in_flight_timeout(&mut self, now: Instant, timeout: Duration) {
        if self.in_flight {
            if let Some(since) = self.in_flight_since {
                if now.duration_since(since) >= timeout {
                    log::debug!(
                        "[coalesce] In-flight update timed out after {:.1?}, releasing in-flight lock",
                        since.elapsed()
                    );
                    self.in_flight = false;
                    self.in_flight_since = None;
                }
            }
        }
    }
}

pub struct UpdateCoalescer {
    min_interval: Duration,
    in_flight_timeout: Duration,
    entries: HashMap<ResourceLink, CoalesceEntry>,
}

impl UpdateCoalescer {
    pub const DEFAULT_IN_FLIGHT_TIMEOUT: Duration = Duration::from_millis(1200);

    #[must_use]
    pub fn new(min_interval: Duration) -> Self {
        Self {
            min_interval,
            in_flight_timeout: Self::DEFAULT_IN_FLIGHT_TIMEOUT,
            entries: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_in_flight_timeout(mut self, timeout: Duration) -> Self {
        self.in_flight_timeout = timeout;
        self
    }

    /// Submits a LightUpdate.
    /// If the rate limit and in-flight status allow immediate sending, returns `Some(to_send)`.
    /// (If there was a pending update, it is merged with `incoming` and returned).
    /// Otherwise, coalesces the update with any existing pending update and returns `None`.
    pub fn submit_light(
        &mut self,
        link: &ResourceLink,
        incoming: LightUpdate,
    ) -> Option<LightUpdate> {
        if self.min_interval.is_zero() {
            return Some(incoming);
        }

        let now = Instant::now();
        let min_interval = self.min_interval;
        let in_flight_timeout = self.in_flight_timeout;
        let entry = self
            .entries
            .entry(link.clone())
            .or_insert_with(|| CoalesceEntry {
                last_sent: now.checked_sub(min_interval).unwrap_or(now),
                pending: None,
                deadline: None,
                in_flight: false,
                in_flight_since: None,
                coalesced_count: 0,
                rolling_latency: None,
            });

        entry.check_in_flight_timeout(now, in_flight_timeout);
        let effective_interval = entry.effective_interval(min_interval);

        if !entry.in_flight && (now >= entry.last_sent + effective_interval) {
            entry.last_sent = now;
            entry.deadline = None;
            entry.in_flight = true;
            entry.in_flight_since = Some(now);
            let count = entry.coalesced_count;
            entry.coalesced_count = 0;

            if let Some(PendingUpdate::Light(mut existing)) = entry.pending.take() {
                existing.merge(incoming);
                if count > 0 {
                    log::debug!(
                        "[coalesce] Dispatched Light {:?} (coalesced {} updates)",
                        link,
                        count + 1
                    );
                }
                Some(existing)
            } else {
                Some(incoming)
            }
        } else {
            entry.coalesced_count += 1;
            match &mut entry.pending {
                Some(PendingUpdate::Light(existing)) => {
                    existing.merge(incoming);
                }
                _ => {
                    entry.pending = Some(PendingUpdate::Light(incoming));
                }
            }
            if entry.deadline.is_none() {
                entry.deadline = Some(entry.last_sent + effective_interval);
            }
            None
        }
    }

    /// Submits a GroupedLightUpdate.
    /// If the rate limit and in-flight status allow immediate sending, returns `Some(to_send)`.
    /// (If there was a pending update, it is merged with `incoming` and returned).
    /// Otherwise, coalesces the update with any existing pending update and returns `None`.
    pub fn submit_grouped_light(
        &mut self,
        link: &ResourceLink,
        incoming: GroupedLightUpdate,
    ) -> Option<GroupedLightUpdate> {
        if self.min_interval.is_zero() {
            return Some(incoming);
        }

        let now = Instant::now();
        let min_interval = self.min_interval;
        let in_flight_timeout = self.in_flight_timeout;
        let entry = self
            .entries
            .entry(link.clone())
            .or_insert_with(|| CoalesceEntry {
                last_sent: now.checked_sub(min_interval).unwrap_or(now),
                pending: None,
                deadline: None,
                in_flight: false,
                in_flight_since: None,
                coalesced_count: 0,
                rolling_latency: None,
            });

        entry.check_in_flight_timeout(now, in_flight_timeout);
        let effective_interval = entry.effective_interval(min_interval);

        if !entry.in_flight && (now >= entry.last_sent + effective_interval) {
            entry.last_sent = now;
            entry.deadline = None;
            entry.in_flight = true;
            entry.in_flight_since = Some(now);
            let count = entry.coalesced_count;
            entry.coalesced_count = 0;

            if let Some(PendingUpdate::GroupedLight(mut existing)) = entry.pending.take() {
                existing.merge(incoming);
                if count > 0 {
                    log::debug!(
                        "[coalesce] Dispatched GroupedLight {:?} (coalesced {} updates)",
                        link,
                        count + 1
                    );
                }
                Some(existing)
            } else {
                Some(incoming)
            }
        } else {
            entry.coalesced_count += 1;
            match &mut entry.pending {
                Some(PendingUpdate::GroupedLight(existing)) => {
                    existing.merge(incoming);
                }
                _ => {
                    entry.pending = Some(PendingUpdate::GroupedLight(incoming));
                }
            }
            if entry.deadline.is_none() {
                entry.deadline = Some(entry.last_sent + effective_interval);
            }
            None
        }
    }

    /// Called when an acknowledgment / bridge device update is received for a resource.
    /// Clears the in-flight state and updates the rolling latency measurement.
    pub fn notify_ack(&mut self, link: &ResourceLink) {
        let now = Instant::now();
        let min_interval = self.min_interval;
        let Some(entry) = self.entries.get_mut(link) else {
            return;
        };

        let latency = entry.in_flight_since.map(|s| now.duration_since(s));
        entry.in_flight = false;
        entry.in_flight_since = None;

        if let Some(lat) = latency {
            let ema = match entry.rolling_latency {
                Some(prev) => prev.mul_f32(0.7) + lat.mul_f32(0.3),
                None => lat,
            };
            entry.rolling_latency = Some(ema);

            if lat >= Duration::from_millis(300) {
                log::warn!(
                    "[coalesce] High Zigbee latency for {link:?}: round-trip took {:.1?}, backing off pace to {:.1?}",
                    lat,
                    entry.effective_interval(min_interval)
                );
            } else {
                log::trace!(
                    "[coalesce] Ack for {link:?}: latency={:.1?}, rolling_avg={:.1?}",
                    lat,
                    ema
                );
            }
        }

        // If an update is pending, ensure deadline reflects current interval
        if entry.pending.is_some() {
            let effective_interval = entry.effective_interval(min_interval);
            entry.deadline = Some(entry.last_sent + effective_interval);
        }
    }

    /// Clears the in-flight flag for a resource (e.g. if sending failed).
    pub fn clear_in_flight(&mut self, link: &ResourceLink) {
        if let Some(entry) = self.entries.get_mut(link) {
            entry.in_flight = false;
            entry.in_flight_since = None;
        }
    }

    /// Returns the earliest deadline among all pending updates, if any.
    #[must_use]
    pub fn earliest_deadline(&self) -> Option<Instant> {
        let min_interval = self.min_interval;
        let in_flight_timeout = self.in_flight_timeout;
        self.entries
            .values()
            .filter_map(|e| {
                if e.pending.is_none() {
                    return None;
                }
                if e.in_flight {
                    let effective_interval = e.effective_interval(min_interval);
                    let rate_deadline = e.last_sent + effective_interval;
                    let timeout_deadline = e.in_flight_since.map(|s| s + in_flight_timeout);
                    match timeout_deadline {
                        Some(t) => Some(t.max(rate_deadline)),
                        None => Some(rate_deadline),
                    }
                } else {
                    let effective_interval = e.effective_interval(min_interval);
                    Some(e.deadline.unwrap_or(e.last_sent + effective_interval))
                }
            })
            .min()
    }

    /// Returns true if any updates are currently pending.
    #[must_use]
    pub fn has_pending(&self) -> bool {
        self.entries.values().any(|e| e.pending.is_some())
    }

    /// Takes all pending updates whose deadline has passed and are not blocked by in-flight.
    pub fn take_ready(&mut self) -> Vec<(ResourceLink, PendingUpdate)> {
        let now = Instant::now();
        let min_interval = self.min_interval;
        let in_flight_timeout = self.in_flight_timeout;
        let mut ready = Vec::new();
        for (link, entry) in &mut self.entries {
            if entry.pending.is_none() {
                continue;
            }

            entry.check_in_flight_timeout(now, in_flight_timeout);
            if entry.in_flight {
                continue;
            }

            let effective_interval = entry.effective_interval(min_interval);
            if now >= entry.last_sent + effective_interval {
                if let Some(pending) = entry.pending.take() {
                    entry.last_sent = now;
                    entry.deadline = None;
                    entry.in_flight = true;
                    entry.in_flight_since = Some(now);
                    let count = entry.coalesced_count;
                    entry.coalesced_count = 0;
                    if count > 0 {
                        log::debug!(
                            "[coalesce] Flushed pending {:?} (coalesced {} updates)",
                            link,
                            count
                        );
                    }
                    ready.push((link.clone(), pending));
                }
            } else {
                entry.deadline = Some(entry.last_sent + effective_interval);
            }
        }
        ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hue::api::{On, RType};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_immediate_then_coalesce() {
        let mut coalescer = UpdateCoalescer::new(Duration::from_millis(50));
        let link = RType::Light.link_to(Uuid::new_v4());

        // First update should go through immediately
        let upd1 = LightUpdate::new().with_brightness(Some(10.0));
        let res1 = coalescer.submit_light(&link, upd1);
        assert!(res1.is_some());
        assert_eq!(res1.unwrap().dimming.unwrap().brightness, 10.0);
        assert!(!coalescer.has_pending());

        // Ack first update to clear in-flight
        coalescer.notify_ack(&link);

        // Second update within cooldown should be buffered
        let upd2 = LightUpdate::new().with_brightness(Some(20.0));
        let res2 = coalescer.submit_light(&link, upd2);
        assert!(res2.is_none());
        assert!(coalescer.has_pending());

        // Third update within cooldown should merge into second
        let upd3 = LightUpdate::new()
            .with_brightness(Some(30.0))
            .with_on(Some(On::new(true)));
        let res3 = coalescer.submit_light(&link, upd3);
        assert!(res3.is_none());

        // Wait for cooldown to expire
        tokio::time::sleep(Duration::from_millis(55)).await;

        let ready = coalescer.take_ready();
        assert_eq!(ready.len(), 1);
        let (rlink, pending) = &ready[0];
        assert_eq!(rlink, &link);
        if let PendingUpdate::Light(final_upd) = pending {
            assert_eq!(final_upd.dimming.as_ref().unwrap().brightness, 30.0);
            assert_eq!(final_upd.on.as_ref().unwrap().on, true);
        } else {
            panic!("Expected PendingUpdate::Light");
        }

        assert!(!coalescer.has_pending());
    }

    #[tokio::test]
    async fn test_grouped_light_continuous_updates() {
        let mut coalescer = UpdateCoalescer::new(Duration::from_millis(50));
        let link = RType::GroupedLight.link_to(Uuid::new_v4());

        // First update immediate
        let upd1 = GroupedLightUpdate::new().with_brightness(Some(15.0));
        assert!(coalescer.submit_grouped_light(&link, upd1).is_some());
        coalescer.notify_ack(&link);

        // Rapid updates coalesce
        let upd2 = GroupedLightUpdate::new().with_brightness(Some(25.0));
        assert!(coalescer.submit_grouped_light(&link, upd2).is_none());
        let upd3 = GroupedLightUpdate::new().with_brightness(Some(35.0));
        assert!(coalescer.submit_grouped_light(&link, upd3).is_none());

        // Cooldown passes, next incoming update dispatches the merged buffer immediately!
        tokio::time::sleep(Duration::from_millis(55)).await;
        let upd4 = GroupedLightUpdate::new().with_brightness(Some(45.0));
        let dispatched = coalescer.submit_grouped_light(&link, upd4);
        assert!(dispatched.is_some());
        assert_eq!(
            dispatched.unwrap().dimming.as_ref().unwrap().brightness,
            45.0
        );
    }

    #[tokio::test]
    async fn test_in_flight_gating() {
        let mut coalescer = UpdateCoalescer::new(Duration::from_millis(20));
        let link = RType::GroupedLight.link_to(Uuid::new_v4());

        // First update dispatches and sets in-flight
        let upd1 = GroupedLightUpdate::new().with_brightness(Some(10.0));
        assert!(coalescer.submit_grouped_light(&link, upd1).is_some());

        // Even after rate limit interval passes, if still in flight, next update is held
        tokio::time::sleep(Duration::from_millis(30)).await;
        let upd2 = GroupedLightUpdate::new().with_brightness(Some(20.0));
        assert!(coalescer.submit_grouped_light(&link, upd2).is_none());
        assert!(coalescer.take_ready().is_empty());

        // Ack arrives -> now it can be taken
        coalescer.notify_ack(&link);
        let ready = coalescer.take_ready();
        assert_eq!(ready.len(), 1);
        if let PendingUpdate::GroupedLight(u) = &ready[0].1 {
            assert_eq!(u.dimming.as_ref().unwrap().brightness, 20.0);
        } else {
            panic!("Expected PendingUpdate::GroupedLight");
        }
    }
}
