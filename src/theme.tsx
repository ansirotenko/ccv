import { createTheme } from '@mui/material/styles';
import { red } from '@mui/material/colors';

const theme = createTheme({
    palette: {
        primary: {
            main: '#1c79fd',
        },
        secondary: {
            main: '#6dbbb4',
        },
        error: {
            main: red.A400,
        },
    },
});

export default theme;
