import React from 'react';
import { Paper, Typography, Button, CircularProgress } from '@mui/material';
import { UploadCloud } from 'lucide-react';

interface UploadZoneProps {
    loading: boolean;
    isDragging: boolean;
    onDragOver: (e: React.DragEvent) => void;
    onDragLeave: (e: React.DragEvent) => void;
    onDrop: (e: React.DragEvent) => void;
    onFileSelect: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

const UploadZone: React.FC<UploadZoneProps> = ({
    loading,
    isDragging,
    onDragOver,
    onDragLeave,
    onDrop,
    onFileSelect
}) => {
    return (
        <Paper
            elevation={0}
            onDragOver={onDragOver}
            onDragLeave={onDragLeave}
            onDrop={onDrop}
            sx={{
                p: 8,
                border: '2px dashed',
                borderColor: isDragging ? 'primary.main' : 'divider',
                borderRadius: 4,
                textAlign: 'center',
                bgcolor: isDragging ? 'rgba(16, 185, 129, 0.05)' : 'background.paper',
                transition: 'all 0.2s ease-in-out',
                position: 'relative',
                overflow: 'hidden'
            }}
        >
            <UploadCloud size={64} color={isDragging ? "#10b981" : "#64748b"} style={{ marginBottom: 16 }} />
            <Typography variant="h4" color="text.primary" gutterBottom>
                myDATA Validator Pro
            </Typography>
            <Typography color="text.secondary" sx={{ mb: 4, maxWidth: 400, mx: 'auto' }}>
                {isDragging 
                    ? "Αφήστε τα αρχεία για έλεγχο" 
                    : "Σύρετε αρχεία XML εδώ ή επιλέξτε από τον υπολογιστή σας για άμεση επικύρωση"}
            </Typography>

            <input
                accept=".xml"
                style={{ display: 'none' }}
                id="upload-xml-input"
                type="file"
                multiple
                onChange={onFileSelect}
            />
            <label htmlFor="upload-xml-input">
                <Button
                    variant="contained"
                    component="span"
                    size="large"
                    disabled={loading}
                    sx={{ 
                        px: 8, 
                        py: 2, 
                        borderRadius: 10, 
                        fontSize: '1rem',
                        boxShadow: '0 10px 15px -3px rgba(16, 185, 129, 0.3)'
                    }}
                >
                    {loading ? <CircularProgress size={24} color="inherit" /> : "Επιλογή Αρχείων"}
                </Button>
            </label>
        </Paper>
    );
};

export default UploadZone;
