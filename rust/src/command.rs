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
use crate::client::Client;
use crate::error::ClientError;
use crate::pb::{QueryRouteRequest, QueryRouteResponse, SendMessageRequest, SendMessageResponse};
use tokio::sync::oneshot;
use tonic::{Request, Response};

pub(crate) enum Command {
    QueryRoute {
        peer: String,
        request: Request<QueryRouteRequest>,
        tx: oneshot::Sender<Result<Response<QueryRouteResponse>, ClientError>>,
    },
    Send {
        peer: String,
        request: Request<SendMessageRequest>,
        tx: oneshot::Sender<Result<Response<SendMessageResponse>, ClientError>>,
    },
}
