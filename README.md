# Simple AI-Powered Labeling System

Web-based data labeling platform where users can annotate images with tags.

## Structure

Notation: data being sent is shown above the arrows; the way it is sent is shown below.

Data flow of login and session info:

![](/docs/login_flow.png)

Data flow of image selection:

![](/docs/image_selection_flow.png)

Data flow of image uploading:

![](/docs/image_upload_flow.png)

## Frontend

Using Vite as build tool.

ChatGPT handled the frontend style: it generated the html to be returned by each component. Also, it generated the handlers in Image.tsx, to send the appropriate request to the server from the user inputs.

## Backend

As suggested by Gemini 2.5 Pro, the project structure is based on a Layered Architecture (Handler -> Service -> Repository) and organized by feature, e.g. auth for user authentication.
* Base architecture:
    - handler.rs: handles endpoints, calls appropriate service layer
    - serive.rs: core logics, may call repository layer when handling database
    - repository.rs: handles database
* Other parts/files:
    - mod.rs: declares each module and exports its public parts
    - model.rs: structs

## Databases

The databases are stored in csv files for simplicity. They are organized as follows:
* image_database: image id and url
* tags_database: tag, the image id it refers to, and whether it has been approved (by an admin)
* user_database: user data (username, password - to be encrypted - and whether they are an admin)

The paths to these databases are stored in a .env file.