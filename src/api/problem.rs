use crate::lib::{
    models::*, schema::problems, schema::problems::dsl::*, util::establish_connection, Config,
    Error,
};
use actix_web::{get, post, web, Responder, Result, Scope};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::Write, path::Path};

#[derive(Serialize, Deserialize)]
struct InnerProblem {
    pub title: String,
    pub description: String,
    pub test_cases: Vec<BTreeMap<String, String>>,
    pub sample_code: BTreeMap<String, String>,
}

fn read_problem_content(problem: &Problem) -> Result<InnerProblem, Error> {
    let content_string = fs::read_to_string(&problem.content_path)?;
    let ip = serde_json::from_str::<InnerProblem>(&content_string)?;
    Ok(ip)
}

#[get("/get/{id}")]
async fn get_problem(path: web::Path<i32>) -> Result<impl Responder, Error> {
    let search_id = path.into_inner();
    let db_conn = &mut establish_connection(&Config::load_unsafe());
    let problem = problems
        .filter(id.eq(search_id))
        .first::<Problem>(db_conn)?;
    let contents = read_problem_content(&problem)?;
    Ok(web::Json(contents))
}

#[post("/create")]
async fn create_problem(body: String) -> Result<impl Responder, Error> {
    let config = Config::load_unsafe();
    let new_problem = serde_json::from_str::<InnerProblem>(&body)?;
    let problem_content_path = Path::new(&config.content_root).join(&new_problem.title);
    let mut content_file = fs::File::create(&problem_content_path)?;

    content_file.write_all(&body.as_bytes())?;

    let new_problem = NewProblem {
        title: new_problem.title,
        content_path: problem_content_path.to_str().unwrap().to_string(),
    };

    let db_conn = &mut establish_connection(&config);
    diesel::insert_into(problems::table)
        .values(&new_problem)
        .execute(db_conn)?;
    Ok(web::Json(BTreeMap::from([(
        "message",
        "Problem successfully created",
    )])))
}

pub fn service_group() -> Scope {
    web::scope("/problems")
        .service(create_problem)
        .service(get_problem)
}
