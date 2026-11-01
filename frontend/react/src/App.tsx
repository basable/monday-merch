import { useEffect, useState } from "react";
import { health } from "./api";
import { Auth } from "./Auth";
import { BRAND } from "./brand";
import { Landing } from "./Landing";

export function App() {
  const [apiReady, setApiReady] = useState<boolean | null>(null);
  const [signedIn, setSignedIn] = useState<boolean | null>(null);

  useEffect(() => {
    health.check({}).then(
      (res) => setApiReady(res.ready),
      () => setApiReady(false),
    );
    // The Kratos session, read with the browser's cookie.
    fetch("/.ory/sessions/whoami", { credentials: "include" }).then(
      (res) => setSignedIn(res.ok),
      () => setSignedIn(false),
    );
  }, []);

  const onAuthPage = window.location.pathname.startsWith("/auth/");

  return (
    <>
      <header className="topbar">
        <a className="brand" href="/">
          <span className="brand-mark" aria-hidden="true" />
          {BRAND}
        </a>
        <nav aria-label="Account">
          {signedIn === false && <a href="/.ory/self-service/login/browser">Sign in</a>}
          {signedIn === true && <a href="/.ory/self-service/logout/browser">Sign out</a>}
        </nav>
      </header>
      {onAuthPage ? (
        <main className="panel">
          <Auth />
        </main>
      ) : (
        <Landing apiReady={apiReady} signedIn={signedIn} />
      )}
    </>
  );
}
