import * as React from 'react';
import PropTypes from 'prop-types';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import Divider from '@mui/material/Divider';
import Drawer from '@mui/material/Drawer';
import IconButton from '@mui/material/IconButton';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import MenuIcon from '@mui/icons-material/Menu';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import ToogleButton from './ToogleButton';
import { Link } from 'react-router-dom';
import { useState } from 'react';
import { Stack } from '@mui/material';

// My function for zoom in the logo

function Zoom() {
  const [isZoomed, setIsZoomed] = useState(false);

  function handleClick() {
    setIsZoomed(true);
    setTimeout(() => setIsZoomed(false), 100); // Reset zoom after 1 second
  }

  return (
    <Link onClick={handleClick} to="/" style={{ textDecoration: 'none' }}>
      <span style={{ alignItems:"center", fontSize: isZoomed ? '2em' : '1.5em', transition: 'font-size 0.2s' }}>⚽</span>
    </Link>
  );
}


 // My function for Create a Navbar

const Lnk = ['/','/about','/ligue_A','/ligue_B'];

const drawerWidth = 240;
const navItems = ['Home', 'About', 'Ligue_A', 'Ligue_B'];

function DrawerAppBar(props) {
  const { window } = props;
  const [mobileOpen, setMobileOpen] = React.useState(false);


  const handleDrawerToggle = () => {
    setMobileOpen((prevState) => !prevState);
    console.log({ window });
  };

  const drawer = (
    <Box onClick={handleDrawerToggle} sx={{ textAlign: 'center',color: '#FFF', backgroundColor: '#242424'  }}>
      <Typography variant="h6" sx={{ my: 2 }}>
        Menu
      </Typography>
      <Divider />


      <List>
        {navItems.map((item) => (
            <ListItem key={item} disablePadding>
            <ListItemButton sx={{ textAlign: 'center'}}>
            <Link to={Lnk[navItems.indexOf(item)]} style={{ textDecoration: 'none', color : "White" }}>
                <ListItemText primary={item} />
              </Link>
            </ListItemButton>
          </ListItem>
        ))}
        <Divider />
      </List>



    </Box>
  );

  const container = window !== undefined ? () => window().document.body : undefined;

  return (
    <>
      <CssBaseline />

      <AppBar  position="static" sx={{ color: '#FFF', backgroundColor: '#242424', borderBottom: 'solid 1px #5b5b5b' }} component="nav">

        <Toolbar >
            <IconButton
              color="inherit"
              aria-label="open drawer"
              edge="start"
              onClick={handleDrawerToggle}
              sx={{ mr: 2, display: { sm: 'none' } }}
            >

              <MenuIcon />
            </IconButton>

          <Box sx={{ flexGrow: 1, display: { sm: mobileOpen ? 'block' : 'none'}}} >
            <Zoom/>
          </Box>

          <Typography
            variant="h6"
            component="div"
            sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } ,fontWeight: 'Bold' }}
          >
            <Zoom/>
          </Typography>
          <Box  sx={{ display: { xs: 'none', sm: 'block' } }}>
            <ToogleButton/>
          </Box>
          <Box sx={{ display: { xs: 'none', sm: mobileOpen ? 'none' : 'block' } }}>
            
            {navItems.map((item) => (
                
              <Button disabled={mobileOpen} key={item} sx={{ color: '#FFF', backgroundColor: '#242424' }}>
                <Link to={Lnk[navItems.indexOf(item)]} style={{ textDecoration: 'none', color : "White" }}>
                  {item}
                </Link>
            </Button>
            ))}
          </Box>
        </Toolbar>
      </AppBar>
      <Box component="nav">
        <Drawer
          container={container}
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true, // Better open performance on mobile.
          }}
          sx={{
            display: { xs: 'block', sm: 'none' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
        >
          {drawer}
        </Drawer>

      </Box>

      </>
  );
}


export default DrawerAppBar;
