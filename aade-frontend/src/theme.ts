import { createTheme } from '@mui/material/styles';

export const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#10b981', // Emerald 500
      contrastText: '#ffffff',
    },
    secondary: {
      main: '#6366f1', // Indigo 500
    },
    background: {
      default: '#0f172a', // Slate 900
      paper: '#1e293b',    // Slate 800
    },
    error: {
      main: '#f43f5e', // Rose 500
    },
    warning: {
      main: '#f59e0b', // Amber 500
    },
    success: {
      main: '#10b981',
    },
    text: {
      primary: '#f8fafc',
      secondary: '#94a3b8',
    },
  },
  shape: {
    borderRadius: 12,
  },
  typography: {
    fontFamily: '"Inter", "Noto Sans", sans-serif',
    h4: {
      fontWeight: 800,
      letterSpacing: '-0.02em',
    },
    h6: {
      fontWeight: 700,
    },
    button: {
      textTransform: 'none',
      fontWeight: 600,
    },
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          padding: '10px 24px',
          boxShadow: 'none',
          '&:hover': {
            boxShadow: '0 4px 12px rgba(16, 185, 129, 0.2)',
          },
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundImage: 'none',
          border: '1px solid rgba(255, 255, 255, 0.05)',
        },
      },
    },
    MuiAccordion: {
      styleOverrides: {
        root: {
          backgroundColor: 'transparent',
          '&:before': {
            display: 'none',
          },
        },
      },
    },
  },
});
