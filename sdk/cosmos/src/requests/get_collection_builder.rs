use crate::prelude::*;
use crate::responses::GetCollectionResponse;
use crate::CollectionClientRequired;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    collection_client: &'a dyn CollectionClient<C, D>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, C, D> GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a dyn CollectionClient<C, D>,
    ) -> GetCollectionBuilder<'a, C, D> {
        GetCollectionBuilder {
            collection_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, C, D> CollectionClientRequired<'a, C, D> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn collection_client(&self) -> &'a dyn CollectionClient<C, D> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, D> UserAgentOption<'a> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, C, D> ActivityIdOption<'a> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, C, D> ConsistencyLevelOption<'a> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, C, D> UserAgentSupport<'a> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = GetCollectionBuilder<'a, C, D>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        GetCollectionBuilder {
            collection_client: self.collection_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, D> ActivityIdSupport<'a> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = GetCollectionBuilder<'a, C, D>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        GetCollectionBuilder {
            collection_client: self.collection_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, D> ConsistencyLevelSupport<'a> for GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = GetCollectionBuilder<'a, C, D>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        GetCollectionBuilder {
            collection_client: self.collection_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C, D> GetCollectionBuilder<'a, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub async fn execute(&self) -> Result<GetCollectionResponse, CosmosError> {
        trace!("GetCollectionResponse::execute called");

        let request = self
            .collection_client()
            .prepare_request_with_collection_name(http::Method::GET);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
