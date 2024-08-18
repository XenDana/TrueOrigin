import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { TrustOrigin_backend } from '../../declarations/TrustOrigin_backend';

function App() {
  const [count, setCount] = useState(0)
  const [greeting, setGreeting] = useState('');

  useEffect(() => {
    const doGreeting = async () => {
      const greet = await TrustOrigin_backend.greet('World')
      console.log(greet);
      setGreeting(greet);
    };

    doGreeting();
  }, [])

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
      <p>Greetings: "{greeting}"</p>
    </>
  )
}

export default App
