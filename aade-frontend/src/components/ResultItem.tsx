import React from 'react';
import {
    Box,
    Paper,
    Typography,
    Stack,
    Chip,
    LinearProgress,
    Accordion,
    AccordionSummary,
    AccordionDetails,
    Alert,
    Card,
    CardContent,
    Divider
} from '@mui/material';
import {
    ChevronDown,
    ShieldCheck,
    ShieldAlert,
    AlertTriangle,
    FileText,
    FileX,
    CheckCircle2,
    Info,
    Lightbulb,
    AlertCircle
} from 'lucide-react';
import { BatchFileResult, ValidationStatus } from '../types';
import { getErrorExplanation, getImpactColor, getImpactLabel } from '../utils/errorExplanations';

interface ResultItemProps {
    fileResult: BatchFileResult;
}

const getStatusConfig = (status: ValidationStatus) => {
    switch (status) {
        case ValidationStatus.Green:
            return { color: '#10b981', icon: <ShieldCheck size={28} color="#10b981" />, label: "ΕΓΚΥΡΟ" };
        case ValidationStatus.Yellow:
            return { color: '#f59e0b', icon: <AlertTriangle size={28} color="#f59e0b" />, label: "ΠΡΟΣΟΧΗ" };
        case ValidationStatus.Red:
            return { color: '#f43f5e', icon: <ShieldAlert size={28} color="#f43f5e" />, label: "ΑΚΥΡΟ" };
    }
};

const ResultItem: React.FC<ResultItemProps> = ({ fileResult }) => {
    return (
        <Paper sx={{ borderRadius: 3, overflow: 'hidden', mb: 3, border: '1px solid rgba(255,255,255,0.05)' }}>
            {/* Header */}
            <Box sx={{ p: 2.5, bgcolor: 'rgba(255,255,255,0.02)', display: 'flex', alignItems: 'center', gap: 2 }}>
                {fileResult.status === 'success' ? <FileText size={24} color="#94a3b8" /> : <FileX size={24} color="#f43f5e" />}
                <Typography variant="subtitle1" fontWeight="700" sx={{ flexGrow: 1 }}>
                    {fileResult.filename}
                </Typography>
                {fileResult.status === 'error' && (
                    <Chip label="XML Error" color="error" size="small" variant="outlined" />
                )}
                {fileResult.status === 'success' && (
                    <Chip label={`${fileResult.reports.length} Παραστατικά`} color="primary" variant="outlined" size="small" />
                )}
            </Box>

            {fileResult.error_message && (
                <Alert severity="error" sx={{ m: 2, borderRadius: 2 }}>{fileResult.error_message}</Alert>
            )}

            {/* Reports */}
            <Box sx={{ p: 2 }}>
                {fileResult.reports.map((report, rIdx) => {
                    const config = getStatusConfig(report.status);
                    return (
                        <Accordion 
                            key={rIdx} 
                            disableGutters 
                            elevation={0} 
                            sx={{ 
                                border: '1px solid rgba(255,255,255,0.05)', 
                                mb: 1.5, 
                                borderRadius: '8px !important',
                                '&:before': { display: 'none' } 
                            }}
                        >
                            <AccordionSummary expandIcon={<ChevronDown color="#64748b" />}>
                                <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, width: '100%' }}>
                                    {config.icon}
                                    <Box sx={{ flexGrow: 1 }}>
                                        <Typography variant="body1" fontWeight="700" color={config.color}>
                                            {config.label}
                                        </Typography>
                                        <Typography variant="caption" color="text.secondary">
                                            {report.summary}
                                        </Typography>
                                    </Box>
                                    <Box sx={{ textAlign: 'right', mr: 2 }}>
                                        <Typography variant="caption" fontWeight="bold">ΚΙΝΔΥΝΟΣ: {report.risk_score}%</Typography>
                                        <LinearProgress
                                            variant="determinate"
                                            value={report.risk_score}
                                            sx={{ 
                                                width: 80, 
                                                height: 6, 
                                                borderRadius: 5, 
                                                bgcolor: 'rgba(255,255,255,0.1)', 
                                                '& .MuiLinearProgress-bar': { bgcolor: config.color } 
                                            }}
                                        />
                                    </Box>
                                </Box>
                            </AccordionSummary>
                            <AccordionDetails sx={{ bgcolor: 'rgba(0,0,0,0.1)', p: 2 }}>
                                <Stack spacing={2}>
                                    {report.errors.length === 0 ? (
                                        <Box sx={{ p: 1, display: 'flex', alignItems: 'center', gap: 1 }}>
                                            <CheckCircle2 size={18} color="#10b981" />
                                            <Typography variant="body2" color="#10b981">Δεν βρέθηκαν σφάλματα.</Typography>
                                        </Box>
                                    ) : (
                                        report.errors.map((err, eIdx) => {
                                            const explanation = getErrorExplanation(err.code);
                                            return (
                                                <Card key={eIdx} sx={{ bgcolor: 'background.paper', borderLeft: `4px solid ${getImpactColor(explanation.impact)}` }}>
                                                    <CardContent sx={{ p: 2 }}>
                                                        <Stack direction="row" justifyContent="space-between" sx={{ mb: 1.5 }}>
                                                            <Stack direction="row" spacing={1.5} alignItems="center">
                                                                <Typography variant="h5">{explanation.icon}</Typography>
                                                                <Box>
                                                                    <Typography variant="caption" fontWeight="bold" color="text.secondary">
                                                                        {err.code}
                                                                    </Typography>
                                                                    <Typography variant="subtitle1" fontWeight="700">
                                                                        {explanation.title}
                                                                    </Typography>
                                                                </Box>
                                                            </Stack>
                                                            <Chip
                                                                label={getImpactLabel(explanation.impact)}
                                                                size="small"
                                                                sx={{ bgcolor: `${getImpactColor(explanation.impact)}20`, color: getImpactColor(explanation.impact), fontWeight: 'bold' }}
                                                            />
                                                        </Stack>

                                                        <Typography variant="body2" sx={{ mb: 2 }}>{err.reason}</Typography>

                                                        {err.field && (
                                                            <Box sx={{ mb: 2, p: 1.5, bgcolor: 'rgba(0,0,0,0.2)', borderRadius: 1.5 }}>
                                                                <Stack direction="row" spacing={1} alignItems="center">
                                                                    <AlertCircle size={14} color="#94a3b8" />
                                                                    <Typography variant="caption" color="text.secondary" fontWeight="bold">ΠΕΔΙΟ:</Typography>
                                                                    <Typography variant="caption" sx={{ fontFamily: 'monospace', bgcolor: 'rgba(255,255,255,0.05)', px: 1, py: 0.3, borderRadius: 0.5 }}>
                                                                        {err.field}
                                                                    </Typography>
                                                                </Stack>
                                                            </Box>
                                                        )}

                                                        <Divider sx={{ my: 2, opacity: 0.1 }} />

                                                        <Stack spacing={2}>
                                                            <Box sx={{ display: 'flex', gap: 1.5 }}>
                                                                <Info size={18} color="#6366f1" />
                                                                <Box>
                                                                    <Typography variant="caption" fontWeight="bold" color="#6366f1" display="block">ΕΠΕΞΗΓΗΣΗ</Typography>
                                                                    <Typography variant="body2" color="text.secondary">{explanation.description}</Typography>
                                                                </Box>
                                                            </Box>
                                                            <Box sx={{ display: 'flex', gap: 1.5, p: 1.5, bgcolor: 'rgba(16, 185, 129, 0.05)', borderRadius: 1.5 }}>
                                                                <Lightbulb size={18} color="#10b981" />
                                                                <Box>
                                                                    <Typography variant="caption" fontWeight="bold" color="#10b981" display="block">ΠΡΟΤΕΙΝΟΜΕΝΗ ΛΥΣΗ</Typography>
                                                                    <Typography variant="body2" color="#10b981">{explanation.solution}</Typography>
                                                                </Box>
                                                            </Box>
                                                        </Stack>
                                                    </CardContent>
                                                </Card>
                                            );
                                        })
                                    )}
                                </Stack>
                            </AccordionDetails>
                        </Accordion>
                    );
                })}
            </Box>
        </Paper>
    );
};

export default ResultItem;
