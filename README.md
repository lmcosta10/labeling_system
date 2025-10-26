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

## Backend

As suggested by Gemini 2.5 Pro, the project structure is based on a Layered Architecture (Handler -> Service -> Repository) and organized by feature, e.g. auth for user authentication.
* Base architecture:
    - handler.rs: handles endpoints, calls appropriate service layer
    - serive.rs: core logics, may call repository layer when handling database
    - repository.rs: handles database
* Other parts/files:
    - mod.rs: declares each module and exports its public parts
    - model.rs: structs