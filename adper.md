# Notes
- keeping the directory initialization in config.rs so that we can ensure the directories exist whenever the get_app_directories function is called

# Bridges between frontend and backend 
## files 
- list files    ---> list all the files linked in the file_links dir
- update files  ---> updates the files used as context for the system

## Querying
- query         ---> The user query through chat interface

## Delete Data
- delete all user data

--------------------

# Logic
## Updating Files
- the user can remove and add individual files 
- once the user hits update database
    - the frontend will send in the list of the files needed
    - the program will fetch the list of exisiting files
    - update the links
    - remove unnecessary markdowns
    - add necessary markdowns
    - update the embeddings
    - then the files listed in the frontend will be updated

## Querying
- once the user types a query
    - the frontend will call the query function from backend with the user query
    - the query will be embedded
    - the database will be queried
    - the chunks returned will be send to the LLM through a JSON prompt (Google AI studio through API from .env)
    - the json output from the LLM will be passed to a function on the frontend
    - for now it'll only contain the response to the chat, not summary and vocabulary. Also anonymization will be implemented at a later stage
    - the chat on the UI will be updated

## Deleting the User Data
- when the user prompts the backend to delete the data
    - delete all the file links
    - delete all the markdowns
    - delete the pseudonyms.sqlite
    - delete all the embeddings


--------------------

# File Management
## File Links
- add file link
- remove file link
- list file links

## Markdown
- generate_markdown
- remove_markdown