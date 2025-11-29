from pymongo import MongoClient
from config.settings import settings

client = MongoClient(settings.MONGO_URI)
db = client["gitdigital"]
users = db["users"]
orgs = db["orgs"]
