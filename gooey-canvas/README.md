# Gooey Canvas

This project is built Tauri, Svelte and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Canvas REST API Documentation
You will need to make an api token through canvas.

### Setting up API Key for Project

- [Go to Alamo Profile Settings.](https://alamo.instructure.com/profile/settings)

- Click on the API Access Tokens tab.

- Click on New Access Token.

- Enter a name for the token and click Generate Token.

- Copy the generated token.

- Paste to website

### Using the [canvas api](https://canvas.instructure.com/doc/api/)

#### Prompting from terminal
- curl https://alamo.instructure.com/api/v1/courses/1530834/assignments \
     -H 'Authorization: Bearer *API TOKEN*'

#### Prompting from terminal
- curl https://alamo.instructure.com/api/v1/courses/1530834/assignments \
     -H 'Authorization: Bearer *API TOKEN*'

#### Accessing grade data
https://alamo.instructure.com/api/v1/users/3810399/enrollments

#### Accessing all students enrolled in a course 
https://alamo.instructure.com/api/v1/courses/1530834/enrollments

#### Accessing all active assignments within a course
https://alamo.instructure.com/api/v1/courses/1530834/assignments/14467344

#### Accessing course data 
https://alamo.instructure.com/api/v1/courses/1530834

#### Accessing all past courses
https://alamo.instructure.com/api/v1/courses

#### Accessing grade data for an individual course
https://alamo.instructure.com/api/v1/courses/1530834/assignments/14467354/submissions/3810399

#### Accessing ALL assignments for a course
https://alamo.instructure.com/api/v1/courses/1530834/assignments/


