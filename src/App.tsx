import { useState, useEffect } from "react";
import { UpdateWindow } from "./components/UpdateWindow/UpdateWindow";
import "./App.css";

function App() {
  const [showUpdateWindow, setShowUpdateWindow] = useState(true);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
  }, []);

  const handleUpdateComplete = () => {
    setShowUpdateWindow(false);
    setIsReady(true);
  };

  return (
    <>
      {showUpdateWindow && (
        <UpdateWindow onComplete={handleUpdateComplete} />
      )}
      
      {isReady && (
        <main className="container">
          <h1>Arceus</h1>
        </main>
      )}
    </>
  );
}

export default App;