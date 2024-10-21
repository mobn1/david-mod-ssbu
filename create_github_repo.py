import os
import requests

def create_github_repo(repo_name, description):
    github_token = os.environ.get('GITHUB_TOKEN')
    if not github_token:
        raise ValueError("GITHUB_TOKEN not found in environment variables")

    headers = {
        'Authorization': f'token {github_token}',
        'Accept': 'application/vnd.github.v3+json'
    }

    data = {
        'name': repo_name,
        'description': description,
        'private': False,
        'has_issues': True,
        'has_projects': True,
        'has_wiki': True
    }

    response = requests.post('https://api.github.com/user/repos', headers=headers, json=data)

    if response.status_code == 201:
        print(f"Repository '{repo_name}' created successfully!")
        return response.json()['html_url']
    else:
        print(f"Failed to create repository. Status code: {response.status_code}")
        print(f"Response: {response.text}")
        return None

if __name__ == "__main__":
    repo_name = "david-mod-ssbu"
    description = "David from Cyberpunk Edgerunners mod for Super Smash Bros Ultimate"
    repo_url = create_github_repo(repo_name, description)
    if repo_url:
        print(f"Repository URL: {repo_url}")
