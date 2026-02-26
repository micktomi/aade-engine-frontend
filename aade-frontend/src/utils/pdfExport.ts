import jsPDF from 'jspdf';
import autoTable from 'jspdf-autotable';
import { BatchFileResult, ValidationStatus } from '../types';

const PDF_FONT_FILE = 'NotoSans-Regular.ttf';
const PDF_FONT_NAME = 'NotoSans';
const PDF_FONT_URL = '/fonts/NotoSans-Regular.ttf';

let pdfFontBase64Promise: Promise<string> | null = null;

const arrayBufferToBase64 = (buffer: ArrayBuffer): string => {
    const bytes = new Uint8Array(buffer);
    const chunkSize = 0x8000;
    let binary = '';
    for (let i = 0; i < bytes.length; i += chunkSize) {
        const chunk = bytes.subarray(i, i + chunkSize);
        binary += String.fromCharCode(...chunk);
    }
    return btoa(binary);
};

const ensurePdfUnicodeFont = async (doc: jsPDF): Promise<void> => {
    if (!pdfFontBase64Promise) {
        pdfFontBase64Promise = fetch(PDF_FONT_URL)
            .then(async (response) => {
                if (!response.ok) throw new Error(`Failed to load PDF font: ${response.status}`);
                const buffer = await response.arrayBuffer();
                return arrayBufferToBase64(buffer);
            });
    }
    const fontBase64 = await pdfFontBase64Promise;
    doc.addFileToVFS(PDF_FONT_FILE, fontBase64);
    doc.addFont(PDF_FONT_FILE, PDF_FONT_NAME, 'normal');
    doc.setFont(PDF_FONT_NAME, 'normal');
};

const setUnicodeFont = (doc: jsPDF): void => {
    doc.setFont(PDF_FONT_NAME, 'normal');
};

export const exportResultsToPdf = async (results: BatchFileResult[], stats: any) => {
    const doc = new jsPDF();
    await ensurePdfUnicodeFont(doc);
    
    // Header
    setUnicodeFont(doc);
    doc.setFontSize(18);
    doc.text("Αναφορά Επικύρωσης myDATA", 14, 20);
    setUnicodeFont(doc);
    doc.setFontSize(11);
    doc.text(`Ημερομηνία: ${new Date().toLocaleDateString('el-GR')}`, 14, 28);

    // Stats
    setUnicodeFont(doc);
    doc.text(`Συνολικά Αρχεία: ${stats.total}`, 14, 36);
    setUnicodeFont(doc);
    doc.text(`Έγκυρα: ${stats.passed}`, 14, 42);
    setUnicodeFont(doc);
    doc.text(`Άκυρα: ${stats.failed}`, 60, 42);

    let yPos = 55;

    results.forEach((file) => {
        setUnicodeFont(doc);
        doc.setFontSize(12);
        doc.setTextColor(0, 0, 0);
        doc.text(`Αρχείο: ${file.filename}`, 14, yPos);
        yPos += 7;

        if (file.status === 'error') {
            setUnicodeFont(doc);
            doc.setTextColor(200, 0, 0);
            doc.text(`Σφάλμα: ${file.error_message}`, 14, yPos);
            yPos += 10;
        } else {
            file.reports.forEach((report, idx) => {
                setUnicodeFont(doc);
                doc.setFontSize(10);
                const statusText = report.status === ValidationStatus.Green ? "ΕΓΚΥΡΟ" : (report.status === ValidationStatus.Red ? "ΑΚΥΡΟ" : "ΠΡΟΣΟΧΗ");
                doc.text(`Παραστατικό #${idx + 1}: ${statusText} (Κίνδυνος: ${report.risk_score}%)`, 14, yPos);
                yPos += 7;

                if (report.errors.length > 0) {
                    const tableData = report.errors.map(e => [e.code, e.reason, e.field || '-']);
                    autoTable(doc, {
                        startY: yPos,
                        head: [['Κωδικός', 'Σφάλμα', 'Πεδίο']],
                        body: tableData,
                        margin: { left: 14 },
                        theme: 'striped',
                        styles: { font: PDF_FONT_NAME, fontStyle: 'normal' },
                        headStyles: { font: PDF_FONT_NAME, fontStyle: 'normal' },
                        bodyStyles: { font: PDF_FONT_NAME, fontStyle: 'normal' },
                    });
                    setUnicodeFont(doc);
                    // @ts-ignore
                    yPos = doc.lastAutoTable.finalY + 10;
                } else {
                    yPos += 5;
                }
            });
        }

        if (yPos > 250) {
            doc.addPage();
            yPos = 20;
        }
    });

    doc.save(`aade_report_${new Date().getTime()}.pdf`);
};
