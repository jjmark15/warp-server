use std::net::{IpAddr, Ipv4Addr};

use tokio::task::JoinHandle;

use quiz_domain::QuizServiceImpl;
use server::App;

use crate::common::state::web::RequestBuilder;

pub(crate) mod web;

#[derive(Debug)]
pub(crate) struct TestState {
    request_builder: RequestBuilder,
    server_proc_handle: JoinHandle<()>,
    server_app: App,
}

impl TestState {
    pub(crate) fn request_builder(&mut self) -> &mut RequestBuilder {
        &mut self.request_builder
    }

    fn spawn_server_process() -> (JoinHandle<()>, App) {
        let (app, future) =
            server::App::from_ip_and_port::<QuizServiceImpl>(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
        (
            tokio::spawn(async move {
                future.await;
            }),
            app,
        )
    }

    pub(crate) fn server_http_address(&self) -> String {
        format!("http://{}", self.server_app.socket_address().to_string())
    }
}

impl Default for TestState {
    fn default() -> Self {
        let (join_handle, server_app) = Self::spawn_server_process();
        TestState {
            request_builder: RequestBuilder::default(),
            server_proc_handle: join_handle,
            server_app,
        }
    }
}
