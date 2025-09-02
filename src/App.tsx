import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

function App() {
  const [notifications, setNotifications] = useState<string[]>([]);

  useEffect(() => {
    const unlisten = listen<string>("new-notification", (event) => {
      setNotifications((prev) => [...prev, event.payload]);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div>
      <h1>Notifications</h1>
      <ul>
        {notifications.map((notification, index) => (
          <li key={index}>{notification}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;
