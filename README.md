# gitrekt
wtf is this?

How to get into the monopain?
```
git clone https://github.com/Hacksore/how-to-use-eslint.git
cd how-to-use-eslint
git checkout cursed
yarn && yarn build
git checkout main
```

Now you are left with this 😞

![image](https://user-images.githubusercontent.com/996134/223494979-d720c87e-9f46-486a-8cb8-4acd04e6d588.png)


tl;dr dont use this rust code instead use a bash function as told by the prophet @theprimagen so here it is.

```bash
function gitclean {
  # Find all directories that are not ignored by git and store them in an array
  ignored_dirs=($(git ls-files --others --exclude-standard --directory))
  # Loop through all directories that are not ignored by git
  for dir in "${ignored_dirs[@]}"; do
    # Check if the directory contains any non-gitignored files/folders
    if [[ -z $(git ls-files --directory "$dir") ]]; then
      # If the directory contains only gitignored files/folders, remove it
      echo "removing $dir"
      rm -rf "$dir"
    fi
  done
}
```
