from fastapi import FastAPI, UploadFile, File
from pydantic import BaseModel

app = FastAPI()

class SuggestResponse(BaseModel):
    suggestion: str

@app.post("/suggest", response_model=SuggestResponse)
async def suggest(file: UploadFile = File(...)):
    contents = await file.read()

    # TODO; Placeholder:
    return {"suggestion": "Placeholder suggestion"}