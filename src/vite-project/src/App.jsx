import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

// We import our element here
import NavBar from './element/Navbar/Navbar'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
     <NavBar></NavBar>
    </div>
  )
}

export default App
