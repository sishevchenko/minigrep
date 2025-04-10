use std::{fs, path::PathBuf};


struct MiniGrep {
    _query: String,
    _path: PathBuf,
}


impl MiniGrep {
    fn new(args: &[String]) -> Self {
        let _query = args.get(1).expect("Для работы требуется строка запроса").clone();
        let path_string = args.get(2).expect("Для работы необходимо указать путь до файла или директории для поиска").clone();
        let _path = PathBuf::from(path_string);
        Self {
            _query,
            _path,
        }
    }

    fn search(&self) -> String {
        let mut res = Vec::<String>::new();
        self._search(&self._path, &mut res);
        return res.join("\n");
    }

    fn _search(&self, path: &PathBuf, buff: &mut Vec<String>) {
        if !path.exists() {
            return;
        }
        if path.is_dir() {
            self._search_in_dir(path, buff);
        } else if path.is_file() {
            self._search_in_file(path, buff);
        } else if path.is_symlink() {
            println!("{} является симлинком и не поддерживается", path.to_str().unwrap());
        }
    }

    fn _search_in_dir(&self, path: &PathBuf, buff: &mut Vec<String>) {
        match path.read_dir() {
            Ok(dir) => for entry in dir {
                if let Ok(entry) = entry {
                    self._search(&entry.path(), buff);
                }
            },
            Err(err) => println!(
                "При чтении директории {} возникла ошибка: {}",
                path.to_str().unwrap_or(""),
                err.to_string()
            ),
        }
        
    }

    fn _search_in_file(&self, path: &PathBuf, buff: &mut Vec<String>) {
        let file_content = fs::read_to_string(path).expect("Не удалось прочитать файл");
        for (i, line) in file_content.lines().enumerate() {
            if line.contains(&self._query) {
                let file_path = path.to_str().unwrap();
                let line_num = i + 1;
                let match_line = format!("{file_path}: строка {line_num}: {line}");
                buff.push(match_line);
            }
        }
    }
}


pub fn run(args: &[String]) -> String {
    let grep = MiniGrep::new(args);
    return grep.search();
}
