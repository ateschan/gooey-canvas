# Gooey Canvas

This project is built in Tauri, using Svelte and TypeScript with Vite.

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

Each user has a unique id, when you are calling the api from your browser you will need to replace
my {user_id} with yours by calling https://alamo.instructure.com/api/v1/courses and looking under 0: enrollments: 0: user_id:

Same goes for assingment and course data.

#### Prompting from terminal
- curl https://alamo.instructure.com/api/v1/courses/{course_id}/assignments \
     -H 'Authorization: Bearer {API TOKEN} '

#### Accessing grade data
https://alamo.instructure.com/api/v1/users/{user_id}/enrollments

#### Accessing all current courses
https://alamo.instructure.com/api/v1/courses/{course_id}/enrollments

#### Accessing all past courses
https://alamo.instructure.com/api/v1/courses

#### Accessing all active assignments within a course
https://alamo.instructure.com/api/v1/courses/{course_id}/assignments/{assignment_id}

#### Accessing course data 
https://alamo.instructure.com/api/v1/courses/{course_id}

#### Accessing grade data for an individual assignment
Unfortunately, accessing grade data directly is inaccessable to student accounts direclty though the assignment endpoint,
so we will be having to look at the submission data to determine grades for assignments.

https://alamo.instructure.com/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}

#### Accessing ALL assignments for a course
https://alamo.instructure.com/api/v1/courses/{user_id}/assignments/


