#!/bin/bash
# test_all_xmls.sh - Comprehensive test suite runner for AADE Validator

# Configuration
API_URL="http://localhost:5173/validate"
TEST_DIR="$(dirname "$0")"
RESULTS_FILE="test_results_$(date +%Y%m%d_%H%M%S).log"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
total_tests=0
passed_tests=0
failed_tests=0

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🧪 AADE Validator - Comprehensive Test Suite"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📁 Test Directory: $TEST_DIR"
echo "🌐 API Endpoint: $API_URL"
echo "📝 Results Log: $RESULTS_FILE"
echo ""

# Check if API is running
echo -n "🔍 Checking if validator is running... "
if ! curl -s "$API_URL" > /dev/null 2>&1; then
    if ! curl -s "http://localhost:5173/health" > /dev/null 2>&1; then
        echo -e "${RED}✗ FAILED${NC}"
        echo ""
        echo "❌ Error: AADE Validator is not running on localhost:5173"
        echo "   Start the validator with: cargo run"
        exit 1
    fi
fi
echo -e "${GREEN}✓ OK${NC}"
echo ""

# Initialize results log
echo "AADE Validator Test Results - $(date)" > "$RESULTS_FILE"
echo "=========================================" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Function to test a single XML file
test_xml_file() {
    local xml_file=$1
    local filename=$(basename "$xml_file")
    local expected_result=$2  # "valid" or "invalid"
    
    total_tests=$((total_tests + 1))
    
    echo -n "  Testing: ${BLUE}$filename${NC}"
    
    # Pad filename for alignment
    local padding=$((60 - ${#filename}))
    printf '%*s' $padding | tr ' ' '.'
    
    # Send XML to validator
    response=$(curl -s -X POST "$API_URL" \
        -H "Content-Type: application/xml" \
        --data-binary "@$xml_file" 2>&1)
    
    # Check if request was successful
    if [ $? -ne 0 ]; then
        echo -e " ${RED}✗ REQUEST FAILED${NC}"
        echo "  ↳ $response"
        echo "$filename: REQUEST FAILED - $response" >> "$RESULTS_FILE"
        failed_tests=$((failed_tests + 1))
        return
    fi
    
    # Parse response
    if echo "$response" | grep -q '"valid":true'; then
        actual_result="valid"
    elif echo "$response" | grep -q '"valid":false'; then
        actual_result="invalid"
    else
        echo -e " ${RED}✗ PARSE ERROR${NC}"
        echo "  ↳ Could not parse response"
        echo "$filename: PARSE ERROR" >> "$RESULTS_FILE"
        failed_tests=$((failed_tests + 1))
        return
    fi
    
    # Compare with expected result
    if [ "$actual_result" == "$expected_result" ]; then
        echo -e " ${GREEN}✓ PASS${NC}"
        echo "$filename: PASS (expected $expected_result, got $actual_result)" >> "$RESULTS_FILE"
        passed_tests=$((passed_tests + 1))
    else
        echo -e " ${RED}✗ FAIL${NC}"
        echo "  ↳ Expected: $expected_result, Got: $actual_result"
        echo "$filename: FAIL (expected $expected_result, got $actual_result)" >> "$RESULTS_FILE"
        
        # Extract error messages if invalid
        if [ "$actual_result" == "invalid" ]; then
            errors=$(echo "$response" | grep -o '"error_code":"[^"]*"' | cut -d'"' -f4 | head -3)
            if [ ! -z "$errors" ]; then
                echo "  ↳ Errors: $(echo $errors | tr '\n' ', ' | sed 's/,$//')"
                echo "     Errors: $errors" >> "$RESULTS_FILE"
            fi
        fi
        
        failed_tests=$((failed_tests + 1))
    fi
    
    # Add separator to log
    echo "" >> "$RESULTS_FILE"
}

# Test Valid XMLs
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Testing VALID Invoices (should pass)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

for xml_file in "$TEST_DIR"/v*.xml; do
    if [ -f "$xml_file" ]; then
        test_xml_file "$xml_file" "valid"
    fi
done

echo ""

# Test Invalid XMLs
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "❌ Testing INVALID Invoices (should fail validation)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

for xml_file in "$TEST_DIR"/i*.xml; do
    if [ -f "$xml_file" ]; then
        test_xml_file "$xml_file" "invalid"
    fi
done

echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 TEST SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  Total Tests:   $total_tests"
echo -e "  ${GREEN}Passed:        $passed_tests${NC}"
echo -e "  ${RED}Failed:        $failed_tests${NC}"
echo ""

# Calculate pass rate
if [ $total_tests -gt 0 ]; then
    pass_rate=$(echo "scale=1; ($passed_tests * 100) / $total_tests" | bc)
    echo "  Pass Rate:     ${pass_rate}%"
fi

echo ""
echo "📝 Detailed results saved to: $RESULTS_FILE"
echo ""

# Write summary to log
echo "SUMMARY" >> "$RESULTS_FILE"
echo "=======" >> "$RESULTS_FILE"
echo "Total Tests: $total_tests" >> "$RESULTS_FILE"
echo "Passed: $passed_tests" >> "$RESULTS_FILE"
echo "Failed: $failed_tests" >> "$RESULTS_FILE"
if [ $total_tests -gt 0 ]; then
    echo "Pass Rate: ${pass_rate}%" >> "$RESULTS_FILE"
fi

# Exit code based on results
if [ $failed_tests -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC} 🎉"
    echo ""
    exit 0
else
    echo -e "${YELLOW}⚠ Some tests failed${NC}"
    echo ""
    exit 1
fi
