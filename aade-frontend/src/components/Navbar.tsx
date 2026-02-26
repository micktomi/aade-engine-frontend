import React from 'react';
import { AppBar, Toolbar, Typography, Container, Box } from '@mui/material';
import { ShieldCheck } from 'lucide-react';

const Navbar: React.FC = () => {
  return (
    <AppBar position="static" className="bg-slate-800">
      <Container maxWidth="xl">
        <Toolbar disableGutters>
          <Box display="flex" alignItems="center" gap={1}>
            <ShieldCheck className="text-white" size={32} />
            <Typography
              variant="h6"
              noWrap
              component="a"
              href="/"
              sx={{
                mr: 2,
                display: { xs: 'none', md: 'flex' },
                fontFamily: 'monospace',
                fontWeight: 700,
                letterSpacing: '.1rem',
                color: 'inherit',
                textDecoration: 'none',
              }}
            >
              AADE VALIDATOR
            </Typography>
          </Box>
        </Toolbar>
      </Container>
    </AppBar>
  );
};

export default Navbar;
