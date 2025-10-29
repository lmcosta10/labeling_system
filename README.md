# Simple AI-Powered Labeling System

Web-based data labeling platform where users can annotate images with tags.

## Set up

There are some steps you need to take if you want to run this yourself:
* Change the .env.example files (there are 2 of them: one in /server, one in /front) to .env files. They set some path variables.

## Using the site

If you want to log in as admin, log in as Bob. If you want to log in as a standard user, log in as Alice. Their data is at server/src/database/user_database.csv.

## Details

### Structure

Notation: data being sent is shown above the arrows; the way it is sent is shown below.

Data flow of login and session info:

![](/docs/login_flow.png)

Data flow of image selection:

![](/docs/image_selection_flow.png)

Data flow of image uploading:

![](/docs/image_upload_flow.png)

### Frontend

Using Vite as build tool.

ChatGPT handled the frontend style: it generated the html to be returned by each component. Also, it generated the handlers in Image.tsx, to send the appropriate request to the server from the user inputs.

### Backend

As suggested by Gemini 2.5 Pro, the project structure is based on a Layered Architecture (Handler -> Service -> Repository) and organized by feature, e.g. auth for user authentication.
* Base architecture:
    - handler.rs: handles endpoints, calls appropriate service layer
    - serive.rs: core logics, may call repository layer when handling database
    - repository.rs: handles database
* Other parts/files:
    - mod.rs: declares each module and exports its public parts
    - model.rs: structs

### Databases

The databases are stored in csv files for simplicity. They are organized as follows:
* image_database: image id and url
* sessions_database: username and token
* tags_database: tag, the image id it refers to, and if the tag is valid (whether it has been approved by an admin)
* tagrequests_database: the image id the operation refers to, operation to be performed (add, edit or delete tag), old tag, new tag, and pending (whether the request is waiting for approval)
* user_database: user data (username, password - to be encrypted - and whether they are an admin)
* imggroup_database: the group(s) each image belongs to
* usergroup_database: the group(s) each user belongs to

The paths to these databases are stored in a .env file.