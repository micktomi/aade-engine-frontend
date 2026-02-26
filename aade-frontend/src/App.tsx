import React from 'react';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import { ThemeProvider, CssBaseline } from '@mui/material';
import { darkTheme } from './theme';

function App() {
  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Layout>
        <Dashboard />
      </Layout>
    </ThemeProvider>
  );
}

export default App;