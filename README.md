# Explore Deploy Strategy

### Overview
- This repository contains a working example of deploying a simplistic multi-container web application to DigitalOcean using Docker Compose.<br><br>
  <kbd>![Animation](https://github.com/user-attachments/assets/d4a194fb-874b-47fb-a04e-7f668ba88066)</kbd>

### Requirements
- Docker Compose 2.X
- DigitalOcean subscription

## Details

### Background
This mono repository contains a simplistic multi-container web application orchestrated via Docker Compose. This web application has no scaling or memory persistence requirements. The underlying logic for each container is a hello world application nested within its own directory; the stack includes:

1. Nginx (Proxy)
2. Vue.js (Frontend)
3. Rust (Backend)

For local development, I’ve configured volumes such that source code on the host machine syncs to the container's file system to streamline setup and keep development OS / dependency version agnostic. 

### Deployment Strategy

This project leverages Docker Compose's [multiple file merge feature](https://docs.docker.com/compose/compose-file/extends/#multiple-compose-files) to manage environment-specific configuration:

- `compose.yaml`: Defines shared infrastructure such as networks, volumes, and base service definitions.
- `compose.dev.yaml` and `compose.prod.yaml`: Extend and override the base file to apply development or production-specific settings.

### Running Locally
With Docker Compose installed, execute the following command:
<br>
```console
docker compose --env-file .env.dev -f compose.yaml -f compose.dev.yaml up
```

| URL Path               | Container                         |
|------------------------|-----------------------------------|
| https://localhost/api/ | Nginx route to backend container  |
| https://localhost/     | Nginx route to frontend container |

### DigitalOcean Deployment

Deployment to a DigitalOcean Droplet is automated using GitHub Actions:

1. The following values were added to the repository's secrets:
    - `BACKEND_IMAGE`
    - `BACKEND_PORT`
    - `DROPLET_IP`
    - `DROPLET_SSH_KEY`
    - `DROPLET_USER`
    - `ENVIRONMENT`
    - `FRONTEND_IMAGE`
2. The workflow builds Docker images for both frontend and backend then pushes to GitHub Container Registry (GHCR).
3. The workflow SSHs into the target Droplet using a private key stored in repository secrets.
4. On the Droplet:
   - Workflow logs in to GHCR.
   - The latest images are pulled.
   - The containers are started using `docker compose` with the production configuration.
