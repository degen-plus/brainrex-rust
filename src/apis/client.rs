use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  blockchain_api: Box<::apis::BlockchainApi>,
  crypto_api: Box<::apis::CryptoApi>,
  sentiment_analysis_api: Box<::apis::SentimentAnalysisApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      blockchain_api: Box::new(::apis::BlockchainApiClient::new(rc.clone())),
      crypto_api: Box::new(::apis::CryptoApiClient::new(rc.clone())),
      sentiment_analysis_api: Box::new(::apis::SentimentAnalysisApiClient::new(rc.clone())),
    }
  }

  pub fn blockchain_api(&self) -> &::apis::BlockchainApi{
    self.blockchain_api.as_ref()
  }

  pub fn crypto_api(&self) -> &::apis::CryptoApi{
    self.crypto_api.as_ref()
  }

  pub fn sentiment_analysis_api(&self) -> &::apis::SentimentAnalysisApi{
    self.sentiment_analysis_api.as_ref()
  }


}
