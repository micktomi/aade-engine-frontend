import React, { ReactNode } from 'react';
import Navbar from './Navbar';
import { Container, Box } from '@mui/material';

interface LayoutProps {
  children: ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh', bgcolor: '#f3f4f6' }}>
      <Navbar />
      <Container component="main" maxWidth="lg" sx={{ flexGrow: 1, py: 4 }}>
        {children}
      </Container>
      <Box component="footer" sx={{ py: 3, textAlign: 'center', bgcolor: '#e5e7eb' }}>
        <p className="text-gray-600 text-sm">
          AADE Validation Engine © {new Date().getFullYear()}
        </p>
      </Box>
    </Box>
  );
};

export default Layout;
