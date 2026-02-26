import React, { useState, useMemo } from 'react';
import {
    Box,
    Typography,
    Button,
    Alert,
    Stack,
    Divider,
    Container
} from '@mui/material';
import { Download, RotateCcw } from 'lucide-react';

import { validateBatchXmls } from '../services/api';
import { ValidationStatus } from '../types';
import type { BatchFileResult } from '../types';
import { exportResultsToPdf } from '../utils/pdfExport';

// Components
[
    './UploadZone',
    './StatsCards',
    './ResultItem' // I will create this next
].forEach(() => {}); 

// Temporary local import of ResultItem logic until created
import ResultItem from '../components/ResultItem';
import UploadZone from '../components/UploadZone';
import StatsCards from '../components/StatsCards';

const Dashboard: React.FC = () => {
    const [loading, setLoading] = useState(false);
    const [results, setResults] = useState<BatchFileResult[] | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [isDragging, setIsDragging] = useState(false);

    const stats = useMemo(() => {
        if (!results) return null;
        let total = results.length;
        let passed = 0;
        let warnings = 0;
        let failed = 0;
        let xmlErrors = 0;

        results.forEach(file => {
            if (file.status === 'error') {
                xmlErrors++;
            } else {
                let fileWorstStatus = ValidationStatus.Green;
                file.reports.forEach(r => {
                    if (r.status === ValidationStatus.Red) fileWorstStatus = ValidationStatus.Red;
                    else if (r.status === ValidationStatus.Yellow && fileWorstStatus !== ValidationStatus.Red) fileWorstStatus = ValidationStatus.Yellow;
                });

                if (fileWorstStatus === ValidationStatus.Green) passed++;
                else if (fileWorstStatus === ValidationStatus.Yellow) warnings++;
                else failed++;
            }
        });

        return { total, passed, warnings, failed, xmlErrors };
    }, [results]);

    const processFiles = async (files: FileList) => {
        if (files && files.length > 0) {
            setLoading(true);
            setError(null);
            try {
                const data = await validateBatchXmls(files);
                setResults(data);
            } catch (err: any) {
                console.error(err);
                setError("Αδυναμία επικοινωνίας με την υπηρεσία ελέγχου.");
            } finally {
                setLoading(false);
            }
        }
    };

    const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
        if (event.target.files) {
            await processFiles(event.target.files);
            event.target.value = ''; 
        }
    };

    const handleDrop = async (e: React.DragEvent) => {
        e.preventDefault();
        setIsDragging(false);
        if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
            await processFiles(e.dataTransfer.files);
        }
    };

    const handleReset = () => {
        setResults(null);
        setError(null);
    };

    const hasOutput = Boolean(results || error);

    return (
        <Container maxWidth="lg" sx={{ py: 6 }}>
            <Stack spacing={4}>
                {/* Hero / Upload Section */}
                <UploadZone 
                    loading={loading}
                    isDragging={isDragging}
                    onDragOver={(e) => { e.preventDefault(); setIsDragging(true); }}
                    onDragLeave={(e) => { e.preventDefault(); setIsDragging(false); }}
                    onDrop={handleDrop}
                    onFileSelect={handleFileUpload}
                />

                {error && <Alert severity="error" variant="filled" sx={{ borderRadius: 2 }}>{error}</Alert>}

                {/* Statistics Cards */}
                {stats && <StatsCards stats={stats} />}

                {/* Actions Bar */}
                {hasOutput && (
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mt: 2 }}>
                        <Typography variant="h6" color="text.primary">
                            {results ? 'Αποτελέσματα Ελέγχου' : 'Ενέργειες'}
                        </Typography>
                        <Stack direction="row" spacing={2}>
                            <Button 
                                variant="outlined"
                                startIcon={<RotateCcw size={18} />}
                                onClick={handleReset}
                                color="warning"
                            >
                                Καθαρισμός
                            </Button>
                            {results && stats && (
                                <Button
                                    variant="contained"
                                    color="secondary"
                                    startIcon={<Download size={18} />}
                                    onClick={() => exportResultsToPdf(results, stats)}
                                >
                                    Εξαγωγή PDF
                                </Button>
                            )}
                        </Stack>
                    </Box>
                )}

                {/* Results List */}
                <Box>
                    {results && results.map((fileResult, idx) => (
                        <ResultItem key={idx} fileResult={fileResult} />
                    ))}
                </Box>
            </Stack>
        </Container>
    );
};

export default Dashboard;
