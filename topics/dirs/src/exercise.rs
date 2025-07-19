// 1. Create a folder named "my_app" inside config_dir()
// ~/.config/my_app or %APPDATA%\my_app

// 2. Inside that folder, create a file "settings.json"

// 3. Write a JSON object to that file:
// { "username": "admin", "autosave": true }

// 4. Read and parse that JSON file back into a struct

// 5. Create a file "db.sqlite" inside data_dir()/my_app/

// 6. Create a file "temp.log" inside cache_dir()/my_app/ and append logs to it

// 7. Check if desktop_dir() exists. If yes, create "hello.txt" with content "Test"

// 8. List all files inside download_dir(), print their file names

// 9. If config_dir() is None, fallback to home_dir()/.my_app/ and create folder

// 10. Write a function: get_my_app_path(kind: &str) -> PathBuf
// kind can be "config", "data", "cache"
// Return full path like ~/.config/my_app/, ~/.local/share/my_app/, etc.

pub fn testing(){
    println!("Success with Exercise");
}