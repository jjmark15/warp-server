use std::marker::PhantomData;
use std::net::SocketAddr;
use std::path::PathBuf;

use warp::Future;

use quiz_domain::QuizServiceInterface;

use crate::application::config::{ApplicationConfig, ConfigFactory};
use crate::application::error::AppStartupError;
use crate::application::web::routes;

pub(crate) mod config;
mod error;
mod logging;
pub(crate) mod web;

#[derive(Debug)]
pub struct App<QuizService: QuizServiceInterface> {
    socket_address: SocketAddr,
    quiz_service: PhantomData<QuizService>,
}

impl<QuizService: 'static + QuizServiceInterface> App<QuizService> {
    pub fn run<CfgFactory>(
        config_factory: CfgFactory,
        config_file_path: PathBuf,
    ) -> Result<(Self, impl Future<Output = ()>), AppStartupError>
    where
        QuizService: QuizServiceInterface,
        CfgFactory: ConfigFactory<Config = ApplicationConfig>,
    {
        let config: ApplicationConfig = config_factory.load(config_file_path)?;
        let intended_socket_address: SocketAddr =
            SocketAddr::new(config.web().address(), config.web().port());
        let server = warp::serve(routes::routes::<'static, QuizService>());
        let (bound_socket_address, future) = server.bind_ephemeral(intended_socket_address);

        Ok((App::new(bound_socket_address), future))
    }

    fn new(socket_address: SocketAddr) -> Self {
        App::<QuizService> {
            socket_address,
            quiz_service: PhantomData::default(),
        }
    }

    pub fn socket_address(&self) -> SocketAddr {
        self.socket_address
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use quiz_domain_mocks::MockQuizService;

    use crate::application::config::ConfigFileReaderError;
    use crate::application::config::MockConfigFactory;
    use crate::ApplicationConfigError;

    use super::*;

    #[test]
    fn fails_to_start_app_if_config_reader_error() {
        let config_path = PathBuf::from("fake");
        let mut mock_config_factory = MockConfigFactory::default();
        mock_config_factory
            .expect_load()
            .with(eq(config_path.clone()))
            .returning(|_| {
                Err(ApplicationConfigError::ConfigReaderError(
                    ConfigFileReaderError::BadConfigData,
                ))
            });

        let result = App::<MockQuizService>::run::<MockConfigFactory<ApplicationConfig>>(
            mock_config_factory,
            config_path,
        );

        match result
            .err()
            .expect("Expected app to fail to start but did not")
        {
            AppStartupError::ConfigError(_config_error) => (),
        }
    }
}
