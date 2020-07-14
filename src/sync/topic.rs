use crate::sync::client::Client;

impl Client {
    // /// publish publishes an item on the supplied topic. The payload type must match
    // /// the payload type on the Topic; otherwise Publish will error.
    // pub fn publish(&self, topic: &Topic, payload: Payload) -> Result<u64, String> {
    //     if !topic.validate_payload(payload) {
    //         return Err("invalid payload".to_string());
    //     }
    //     let bytes = payload.serialize();
    //     let redis_key = topic.redis_key();
    //
    //     let conn = self.redis_client().get_conn()?;
    //     conn.publish(redis_key, bytes)
    // }
    //
    // /// subscribe subscribes to a topic, consuming ordered, typed elements from
    // /// index 0, and sending them to channel ch.
    // ///
    // /// The supplied channel must be buffered, and its type must be a value or
    // /// pointer type matching the topic type. If these conditions are unmet, this
    // /// method will error immediately.
    // ///
    // /// The caller must consume from this channel promptly; failure to do so will
    // /// backpressure the DefaultClient's subscription event loop.
    // pub fn subscribe(&self, topic: Topic) -> Result<Receiver<Payload>, String> {
    //     let (sub_response_sender, sub_response_receiver) = bounded(1000);
    //     let sub = Subscription::new(topic);
    //     let add_sub = AddSubscription::new(
    //         subscription: sub,
    //         sub_response_sender,
    //     );
    //     self.subscription_ch.send(add_sub)?;
    //     Ok(sub_response_receiver)
    // }
}
