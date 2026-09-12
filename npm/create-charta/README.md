# @pirol/create-charta

Start a [Company as Code](https://github.com/Pirol-ai/company-as-code) description in the current
directory:

```
npm init @pirol/charta
npm init @pirol/charta -- --name "Acme GmbH"
```

This creates `company.yaml`, which marks the root of your company description. Everything after
that you write with your agent. Check it any time with `charta validate .`
(`npm i -g @pirol/charta`). Apache-2.0.
