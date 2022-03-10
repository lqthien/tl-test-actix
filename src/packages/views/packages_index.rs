use jelly::prelude::*;
use jelly::Result;

use crate::packages::Package;

/// Returns a list of packages in the system.
pub async fn packages_index(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let packages: Vec<Package> = match Package::get_all(db).await {
        Ok(packages) => {
            packages
        }
        Err(_) => {
            Package::create_sample(db).await?;
            Package::get_all(db).await?
        }
    };

    return request.render(200, "packages/index.html", {
        let mut context = Context::new();
        context.insert("packages", &packages);
        context
    })
}
