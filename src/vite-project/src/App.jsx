import './App.css'
import { Route,Routes } from 'react-router-dom'
import React from 'react'

// We import our element here
import NavBar from './element/Navbar/Navbar.jsx' 

//  import for the prono pages
import Home from './element/Body/Home.jsx'
import Home_v2 from './element/Body/Home_v2'
import About from './element/Body/About'
import Ligue_A from './element/Body/Ligue_A'
import Ligue_B from './element/Body/Ligue_B'


function App() {

  const newLocal = <Routes>
    <Route path="/" element={<Home_v2 />}></Route>
    {/* <Route path="/" element={<Home />}></Route> */}
    <Route path="/about" element={<About />}></Route>
    <Route path="/ligue_A" element={<Ligue_A />}></Route>
    <Route path="/ligue_B" element={<Ligue_B />}></Route>
  </Routes>
  return (
    <div className="App">
     
    

     <NavBar/>
      
     {newLocal}
      
     
     
    </div>
  )
}

export default App
