# Explore Deploy Strategy

### Running
With Docker Compose installed, execute the following command:
  
```Console
docker compose --env-file .env.development up
```
  
| URL Path             | Container          |
|----------------------|--------------------|
| https://localhost/api | Nginx route to backend container  |
| https://localhost/    | Nginx route to frontend container |

<kbd>![Animation](https://github.com/user-attachments/assets/d4a194fb-874b-47fb-a04e-7f668ba88066)</kbd>


### Details
This mono repository contains a simplistic multi-container web application orchestrated via Docker Compose. This web application has no scaling or memory persistence requirements. The underlying logic for each container is a hello world application nested within its own directory; the stack includes:

1. Nginx (Proxy)
2. Vue.js (Frontend)
3. Rust (Backend)

For local development, I’ve configured volumes such that source code on the host machine syncs to the container's file system to streamline setup and keep development OS / dependency version agnostic. 

### Goal 
Adapt a deployment strategy to deploy the app on a Digital Ocean droplet through a CI/CD while preserving the local development aspects.






