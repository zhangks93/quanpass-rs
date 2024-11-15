import requests
def notify(content: str):
    requests.get('https://sctapi.ftqq.com/SCT196705TWvKmvCdFWzuMGZ5xPlfvSVM2.send?title='+ content)
    
if __name__ == "__main__":
    notify('TICKET!!!')