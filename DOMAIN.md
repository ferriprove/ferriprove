# ferriprove.org Domain Configuration

## Current Setup (v0.5.0 placeholder)

Point `ferriprove.org` to the GitHub organization page at `https://github.com/ferriprove`.

### DNS Configuration

Add the following DNS records to your domain registrar:

```
Type: CNAME
Name: @ (or ferriprove.org depending on registrar)
Value: ferriprove.github.io
TTL: 3600 (or default)

Type: CNAME  
Name: www
Value: ferriprove.github.io
TTL: 3600 (or default)
```

### GitHub Pages Configuration

1. Go to the ferriprove organization on GitHub
2. Navigate to Settings → Pages
3. Source: Deploy from a branch
4. Branch: `gh-pages` from the `ferriprove/ferriprove` repository
5. Root directory: `/ (root)`

### Future Website (v0.5.0+)

When ready for a proper website at v0.5.0:

1. Create a `gh-pages` branch in the main repository
2. Build a static site (using Hugo, Jekyll, or similar)
3. Deploy to GitHub Pages
4. Update DNS to point to the custom domain

### Verification

After DNS propagation (typically 1-24 hours), verify:

```bash
dig ferriprove.org
# Should return CNAME pointing to ferriprove.github.io

curl -I https://ferriprove.org
# Should return 301 redirect to GitHub org page
```

## Notes

- GitHub Pages provides free hosting for static sites
- The organization page redirect is a temporary solution
- Custom domain setup requires GitHub Pro for organizations
- HTTPS is automatically provided by GitHub Pages
