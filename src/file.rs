use std::fs;

pub fn get_all_music() -> Vec<String> {

    let paths = fs::read_dir(".\\musics").unwrap();

    let mut paths_list: Vec<String> = Vec::new();

    for path in paths {
        paths_list.push(path.unwrap().file_name().into_string().unwrap());
    }

    return paths_list

}