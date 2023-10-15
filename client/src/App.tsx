/// <reference types='./types.d.ts' />
import { useEffect, useState } from 'react'
import './App.css'

function App() {
  const avatar = "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar_lf92vl.png";

  const [people, setPeople] = useState<Person[]>([]);

  useEffect(() => {
    fetch("http://localhost:4000/people")
      .then((res) => res.json())
      .then(setPeople);
  }, []);

  return (
    <>
      {people.map((person, index) => (
        <div key={index} className='card'>
          <img src={avatar} alt='image avatar' />
          <div className='container'>
            <h4>{person.name}</h4>
            <p>Favourite Food: {person.favourite_food ?? 'Unknown'}</p>
          </div>
        </div>
      ))}
    </>
  )
}

export default App
