/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use slog::Logger;
use tokio::sync::oneshot;
use tokio::task::block_in_place;

use crate::{client, error, models};
use crate::error::ClientError;
use crate::pb::SendMessageResponse;

struct Producer {
    client: client::Client,
}

impl Producer {
    pub async fn send(&self, message: &models::MessageImpl) -> Result<SendMessageResponse, ClientError> {
        self.client.send(message).await
    }
}

impl Producer {
    pub async fn new<T>(logger: Logger, access_point: &str, topics: T) -> Result<Self, error::ClientError>
        where
            T: IntoIterator,
            T::Item: AsRef<str>,
    {
        let client = client::Client::new(logger, access_point)?;

        for _topic in topics.into_iter() {
            client.query_route(_topic.as_ref(), false).await.unwrap();
        }
        Ok(Producer { client })
    }


    pub fn start(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use slog::Drain;

    #[tokio::test]
    async fn test_producer() {
        let drain = slog::Discard;
        let logger = Logger::root(drain, slog::o!());
        let access_point = "127.0.0.1:8081";
        let _producer = Producer::new(logger, access_point, vec!["TopicTest"]).await.unwrap();
        let tag = "TagA";
        let mut keys = Vec::new();
        keys.push(String::from("key1"));
        let message = models::MessageImpl::new("TopicTest", tag, keys, "hello world");

        match _producer.send(&message).await {
            Ok(r) => {
                println!("response: {:?}", r);
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    }
}
