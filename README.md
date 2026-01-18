# Simple AI-Powered Labeling System

Web-based data labeling platform where users can annotate images with tags.

## Set up

There are some steps you need to take if you want to run this yourself:
* Change the .env.example files (there are 2 of them: one in /server, one in /front) to .env files. They set some path variables.

## Using the site

If you want to test the site, you can get the data from the databases in /server/src/database, but, to make it simple:
* Users are alice, bob and charlie
* All of the users' passwords are simply "password"
* alice belongs to group 1
* bob is an admin
* charlie does not belong to any group
* Group 1 has 1 image (rust crab)

## Details

### Frontend

Using Vite as build tool.

The frontend style was built using AI (ChatGPT): it generated the initial classes and css files, which were then customized.

### Backend

As suggested by Gemini 2.5 Pro, the project structure is based on a Layered Architecture (Handler -> Service -> Repository) and organized by feature, e.g. auth for user authentication.
* Base architecture:
    - handler.rs: handles endpoints, calls appropriate service layer
    - service.rs: core logics, may call repository layer when handling database
    - repository.rs: handles database
* Other parts/files:
    - mod.rs: declares each module and exports its public parts
    - model.rs: structs

### Databases

The databases are stored in a SQLite file for simplicity. They are organized as follows:
* images: image id and url
* image_groups: the group(s) each image belongs to
* sessions: username and token
* tags: tag, the image id it refers to, and if the tag is valid (whether it has been approved by an admin)
* tag_requests: the image id the operation refers to, operation to be performed (add, edit or delete tag), old tag, new tag, and pending (whether the request is waiting for approval)
* users: user data (username, password - TODO: encryption - and whether they are an admin)
* user_groups: the group(s) each user belongs to

### AI

AI tag suggestions: coming soon. Currently using mock data.