import React from 'react'
import { Typography, Box, Stack,Button } from '@mui/material'


export default function Home() {
  return (
    <>
    <Box component="main" sx={{ p: 3 ,color: '#FFF', backgroundColor: '#242424' , padding: '10vh'}}>
    <Typography variant="h1" sx={{textAlign: "center"}}> 
    ⚽
    </Typography>
    <Typography variant="h4" sx={{textAlign: "center" , padding: "1vh",fontWeight: 'Bold'}}> 
    Moirasport 
    </Typography>
    <Typography variant="h4" sx={{textAlign: "center" , padding: "1vh"}}> 
    Le prono qui ce trompe jamais
    </Typography>
      <Box sx={{display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
      <Stack direction="row" spacing={2} sx={{padding: '1vh', alignItems: 'center'}}>
      <Button variant="contained">Lien A</Button>
        <Button variant="outlined">Lien B</Button>
      </Stack>
      </Box>
    </Box>
    
    </>

  ) 
}
