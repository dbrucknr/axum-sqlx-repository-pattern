```bash
[ SPA ] ──(1)──>  /auth/login   ──(2)──>  IdP /authorize
   ▲                                         │
   │                                         │ (3) user logs in
   │                                         ▼
[ SPA ] <──(6)──  /  <──(5)──  /auth/callback  <──(4)── IdP
                     |
                     | (exchanges code for tokens)
                     | (creates session)
```
