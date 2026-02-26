import React from 'react';
import { Paper, Typography, Box } from '@mui/material';
import { FileCheck, FileX, AlertTriangle, Files } from 'lucide-react';

interface StatsProps {
    stats: {
        total: number;
        passed: number;
        warnings: number;
        failed: number;
        xmlErrors: number;
    } | null;
}

const StatsCards: React.FC<StatsProps> = ({ stats }) => {
    if (!stats) return null;

    const items = [
        { label: 'ΣΥΝΟΛΟ', value: stats.total, icon: <Files size={20} />, color: '#94a3b8' },
        { label: 'ΕΓΚΥΡΑ', value: stats.passed, icon: <FileCheck size={20} />, color: '#10b981' },
        { label: 'ΑΚΥΡΑ', value: stats.failed, icon: <FileX size={20} />, color: '#f43f5e' },
        { label: 'XML ΣΦΑΛΜΑΤΑ', value: stats.xmlErrors, icon: <AlertTriangle size={20} />, color: '#f59e0b' },
    ];

    return (
        <Box
            sx={{
                display: 'grid',
                gridTemplateColumns: { xs: '1fr', sm: 'repeat(4, minmax(0, 1fr))' },
                gap: 2
            }}
        >
            {items.map((item, idx) => (
                <Box key={idx}>
                    <Paper sx={{ p: 2, display: 'flex', alignItems: 'center', gap: 2, bgcolor: 'background.paper' }}>
                        <Box sx={{ p: 1, borderRadius: 2, bgcolor: `${item.color}15`, color: item.color, display: 'flex' }}>
                            {item.icon}
                        </Box>
                        <Box>
                            <Typography variant="caption" color="text.secondary" fontWeight="600">
                                {item.label}
                            </Typography>
                            <Typography variant="h5" fontWeight="800">
                                {item.value}
                            </Typography>
                        </Box>
                    </Paper>
                </Box>
            ))}
        </Box>
    );
};

export default StatsCards;
