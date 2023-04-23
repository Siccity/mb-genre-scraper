use std::time::Duration;
use std::env;
use std::{thread, time};

use musicbrainz_rs::entity::recording::{Recording, RecordingSearchQuery};
use musicbrainz_rs::entity::release::Release;
use musicbrainz_rs::entity::search::SearchResult;
use musicbrainz_rs::prelude::*;

const API_DELAY: Duration = time::Duration::from_millis(500);

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let artist = &args[1];
    let track = &args[2];
    
    let result = search_recording(&artist, &track).await.unwrap().entities;
    if result.len() == 0 {
        println!("Did not find song");
    } else {
        let mut genres = get_genres_from_recordings(&result).await;
        if genres.len() == 0 {
            println!("Found no genres in recordings..");
            let releases = get_releases_from_recording(&result).await;
            genres = get_genres_from_releases(&releases).await;
        }
        dbg!(genres);
    }

    Ok(())
}

async fn search_recording<'a>(artist: &str, title: &str) -> Result<SearchResult<Recording>, Error> {
    println!("Searching for recordings..");
    // Search for recording
    let query = RecordingSearchQuery::query_builder()
        .recording(title)
        .and()
        .artist(artist)
        .build();
    thread::sleep(API_DELAY);
    let recordings = Recording::search(query).execute().await;
    recordings
}

async fn get_genres_from_recordings(recordings: &Vec<Recording>) -> Vec<String> {
    println!("Searching for genres in recordings..");
    let mut genres: Vec<String> = Vec::new();
    for recording in recordings {
        thread::sleep(API_DELAY);
        dbg!(&recording.id);
        let result = Recording::fetch()
            .id(&recording.id)
            .with_genres()
            .with_tags()
            .execute().await;
        let rec = result.as_ref().unwrap();
        if let Some(g) = &rec.genres {
            let names: Vec<String> = g.into_iter().map(|x| x.name.to_owned()).collect();
            append_unique(&mut genres, names);
        };
        if let Some(g) = &rec.tags {
            let names: Vec<String> = g.into_iter().map(|x| x.name.to_owned()).collect();
            append_unique(&mut genres, names);
        };
    }
    genres
}

async fn get_releases_from_recording(recordings: &Vec<Recording>) -> Vec<Release> {
    println!("Searching for releases..");
    let mut releases: Vec<Release> = Vec::new();
    for recording in recordings {
        thread::sleep(API_DELAY);
        let result = Recording::fetch()
            .id(&recording.id)
            .with_releases()
            .execute().await;
        for release in result.unwrap().releases.unwrap() {
            releases.push(release);
        }
    }
    releases
}

async fn get_genres_from_releases(releases: &Vec<Release>) -> Vec<String> {
    println!("Searching for genres in releases..");
    let mut genres: Vec<String> = Vec::new();
    for release in releases {
        thread::sleep(API_DELAY);
        dbg!(&release.id);
        let result = Release::fetch()
            .id(&release.id)
            .with_genres()
            .with_tags()
            .execute().await;
        let rec = result.as_ref().unwrap();
        if let Some(g) = &rec.genres {
            let names: Vec<String> = g.into_iter().map(|x| x.name.to_owned()).collect();
            append_unique(&mut genres, names);
        };
        if let Some(g) = &rec.tags {
            let names: Vec<String> = g.into_iter().map(|x| x.name.to_owned()).collect();
            append_unique(&mut genres, names);
        };
    }
    genres
}

fn append_unique <T>(existing: &mut Vec<T>, new_entries: Vec<T>) where T:std::cmp::PartialEq {
    for entry in new_entries {
        if !existing.contains(&entry) {
            existing.push(entry);
        }
    }
}
