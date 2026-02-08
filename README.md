# Simple AI-Powered Labeling System

Web-based data labeling platform where users can annotate images with tags.

## Set up

1) Change the .env.example files (there are 2 of them: one in /server/, one in /front/) to .env files.

2) To set up the frontend, got to /front/ and run:
```
npm install
npm run dev
```

3) To set up the backend, got to /server/ and run:
```
docker compose up --build
```

## Using the site

If you want to test the site, you can get the data from the databases in /server/database/, but, to make it simple:
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

As suggested by Gemini 2.5 Pro, the project structure is based on a Layered Architecture (Handler -> Service -> Repository) and organized by feature, e.g. files in /auth/ handle user authentication.
* Base architecture:
    - handler.rs: handles endpoints, calls appropriate service layer
    - service.rs: core logics, may call repository layer when handling database
    - repository.rs: handles database
* Other parts/files:
    - mod.rs: declares each module and exports its public parts
    - model.rs: structs

### Databases

The databases are stored in a SQLite file for simplicity. They are organized as follows:
* groups
* images: image id and url
* image_groups: the group(s) each image belongs to
* sessions: username and token
* tags: tag and the image id it refers to
* tag_requests: the image id the operation refers to, operation to be performed (add, edit or delete tag), old tag (for deleting or editing) and new tag (for addition or editing)
* user_groups: the group(s) each user belongs to
* users: user data (username, password - TODO: encryption - and whether they are an admin)

### AI

AI tag suggestions: coming soon. Currently using mock data.