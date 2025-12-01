from fastapi import FastAPI
from routes import profile, orgs

app = FastAPI(title="GitDigital User Service")

app.include_router(profile.router, prefix="/profile")
app.include_router(orgs.router, prefix="/orgs")

@app.get("/")
def root():
    return {"status": "user service online"}
