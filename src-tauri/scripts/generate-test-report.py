#!/usr/bin/env python3
"""Parse cargo test JSON output and generate a markdown report for PR comments."""

import json
import re
import os


def parse_json_results(filepath):
    """Parse cargo test's --format json output."""
    passed = []
    failed = []
    ignored = []
    timing = {}  # name -> exec_time in ms

    if not os.path.exists(filepath):
        return passed, failed, ignored, timing

    with open(filepath, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue

            event_type = event.get("type", "")

            if event_type == "test" and event.get("event") == "ok":
                name = event.get("name", "unknown")
                exec_time = event.get("exec_time")
                passed.append({"name": name})
                if exec_time is not None:
                    timing[name] = exec_time * 1000  # seconds -> ms

            elif event_type == "test" and event.get("event") == "failed":
                name = event.get("name", "unknown")
                stdout = event.get("stdout", "")
                exec_time = event.get("exec_time")
                failed.append({"name": name, "stdout": stdout})
                if exec_time is not None:
                    timing[name] = exec_time * 1000

            elif event_type == "test" and event.get("event") == "ignored":
                name = event.get("name", "unknown")
                ignored.append({"name": name})

    return passed, failed, ignored, timing


def parse_text_results(filepath):
    """Parse cargo test's human-readable output for summary, individual tests, and timing."""
    summary = {"passed": 0, "failed": 0, "ignored": 0, "measured": 0}
    text_passed = []
    text_failed = []
    text_ignored = []
    perf_details = []  # parsed from [PERF] eprintln lines

    if not os.path.exists(filepath):
        return summary, text_passed, text_failed, text_ignored, perf_details

    with open(filepath, "r") as f:
        text_content = f.read()

    # Parse individual test result lines
    test_line_pattern = re.compile(r"^test\s+(\S+)\s+\.\.\.\s+(ok|FAILED|ignored)", re.MULTILINE)
    for match in test_line_pattern.finditer(text_content):
        name = match.group(1)
        status = match.group(2)
        if status == "ok":
            text_passed.append({"name": name})
        elif status == "FAILED":
            text_failed.append({"name": name, "stdout": ""})
        elif status == "ignored":
            text_ignored.append({"name": name})

    # Parse test summary line
    summary_match = re.search(
        r"test result: (ok|FAILED).*?(\d+) passed.*?(\d+) failed.*?(\d+) ignored.*?(\d+) measured",
        text_content,
    )
    if summary_match:
        summary["passed"] = int(summary_match.group(2))
        summary["failed"] = int(summary_match.group(3))
        summary["ignored"] = int(summary_match.group(4))
        summary["measured"] = int(summary_match.group(5))

    # Parse [PERF] structured output from eprintln
    # Format: [PERF] test_name | duration=Xms | iterations=N | ... | per_call=Yµs
    perf_line_pattern = re.compile(
        r"\[PERF\]\s+(\S+)\s+\|\s+duration=(\d+)ms\s+\|\s+(.*)"
    )
    for match in perf_line_pattern.finditer(text_content):
        name = match.group(1)
        duration_ms = int(match.group(2))
        extra_kv = match.group(3)

        detail = {"name": name, "duration_ms": duration_ms}

        # Parse remaining key=value pairs
        for kv_match in re.finditer(r"(\w+)=(\S+)", extra_kv):
            key = kv_match.group(1)
            value = kv_match.group(2)
            # Remove trailing comma
            value = value.rstrip(",")
            if key in ("per_call", "per_column"):
                detail["per_call_us"] = float(value.rstrip("µs"))
            elif key in ("iterations", "types", "total_calls", "columns"):
                detail[key] = int(value)

        perf_details.append(detail)

    return summary, text_passed, text_failed, text_ignored, perf_details


def categorize_tests(passed, failed):
    """Categorize tests into stability and performance groups."""
    stability_passed = [t for t in passed if "perf" not in t["name"].lower()]
    perf_passed = [t for t in passed if "perf" in t["name"].lower()]
    stability_failed = [t for t in failed if "perf" not in t["name"].lower()]
    perf_failed = [t for t in failed if "perf" in t["name"].lower()]

    return {
        "stability": {
            "passed": len(stability_passed),
            "failed": len(stability_failed),
            "total": len(stability_passed) + len(stability_failed),
            "failed_details": stability_failed,
        },
        "performance": {
            "passed": len(perf_passed),
            "failed": len(perf_failed),
            "total": len(perf_passed) + len(perf_failed),
            "failed_details": perf_failed,
        },
    }


def get_perf_threshold(name):
    """Return the performance threshold in ms for a given test name."""
    if "parse_sql_type" in name:
        return 500
    if "format_cell" in name:
        return 2000
    if "resolve_export" in name:
        return 100
    return None


def make_timing_bar(duration_ms, threshold_ms, max_bar_width=20):
    """Generate a visual timing bar using block characters."""
    if threshold_ms and threshold_ms > 0:
        ratio = min(duration_ms / threshold_ms, 1.0)
    else:
        ratio = 0.0

    filled = int(ratio * max_bar_width)
    empty = max_bar_width - filled

    if ratio <= 0.5:
        color = "🟩"
    elif ratio <= 0.8:
        color = "🟨"
    else:
        color = "🟥"

    bar = color * filled + "⬜" * empty
    return bar


def format_duration(duration_ms):
    """Format duration with appropriate unit."""
    if duration_ms < 1:
        return f"{duration_ms * 1000:.0f}µs"
    elif duration_ms < 1000:
        return f"{duration_ms:.1f}ms"
    else:
        return f"{duration_ms / 1000:.2f}s"


def generate_report(passed, failed, ignored, summary, perf_details, json_timing, categories):
    """Generate the markdown report."""
    total = len(passed) + len(failed)
    pass_rate = (len(passed) / total * 100) if total > 0 else 0

    lines = []
    lines.append("## 🧪 Test Results Report")
    lines.append("")

    # Overall summary
    status_icon = "✅" if len(failed) == 0 else "❌"
    lines.append(
        f"**{status_icon} Total: {total} tests | "
        f"Passed: {len(passed)} | "
        f"Failed: {len(failed)} | "
        f"Pass Rate: {pass_rate:.1f}%**"
    )
    lines.append("")

    # Category breakdown table
    lines.append("| Category | Status | Passed | Total | Rate |")
    lines.append("|----------|--------|--------|-------|------|")

    stab = categories["stability"]
    stab_rate = (stab["passed"] / stab["total"] * 100) if stab["total"] else 0
    stab_status = "✅ PASS" if stab["passed"] == stab["total"] else "❌ FAIL"
    lines.append(
        f"| Stability | {stab_status} | {stab['passed']} | {stab['total']} | {stab_rate:.1f}% |"
    )

    perf = categories["performance"]
    if perf["total"] > 0:
        perf_rate = (perf["passed"] / perf["total"] * 100) if perf["total"] else 0
        perf_status = "✅ PASS" if perf["passed"] == perf["total"] else "❌ FAIL"
        lines.append(
            f"| Performance | {perf_status} | {perf['passed']} | {perf['total']} | {perf_rate:.1f}% |"
        )

    lines.append("")

    # Failed tests detail
    if failed:
        lines.append("### ❌ Failed Tests")
        lines.append("")
        for t in failed:
            lines.append(f"- **{t['name']}**")
            if t.get("stdout"):
                stdout_clean = t["stdout"].strip()
                if len(stdout_clean) > 500:
                    stdout_clean = stdout_clean[-500:]
                lines.append(f"  ```")
                lines.append(f"  {stdout_clean}")
                lines.append(f"  ```")
        lines.append("")

    # Performance tests detail with timing bars and per-call metrics
    perf_tests = [t for t in passed + failed if "perf" in t["name"].lower()]
    if perf_details or perf_tests:
        lines.append("### ⚡ Performance Metrics")
        lines.append("")

        if perf_details:
            # Merge JSON timing into perf_details for tests that have it
            for detail in perf_details:
                name = detail["name"]
                if name in json_timing and "duration_ms" not in detail:
                    detail["duration_ms"] = json_timing[name]

            # Summary: total perf time
            total_perf_ms = sum(d.get("duration_ms", 0) for d in perf_details)
            lines.append(f"**Total benchmark time: {format_duration(total_perf_ms)}**")
            lines.append("")

            # Detailed table with timing bars
            lines.append("| Test | Duration | Per Call | Threshold | Utilization | Status |")
            lines.append("|------|----------|----------|-----------|-------------|--------|")

            for p in perf_details:
                name = p["name"]
                # Shorten name for display
                short_name = name.replace("sql::generator_test::", "")
                duration = p.get("duration_ms", 0)
                threshold = get_perf_threshold(name)

                # Per call metric
                per_call_us = p.get("per_call_us")
                if per_call_us is not None:
                    if per_call_us < 1:
                        per_call_str = f"{per_call_us * 1000:.0f}ns"
                    else:
                        per_call_str = f"{per_call_us:.2f}µs"
                else:
                    per_call_str = "-"

                # Threshold and utilization
                if threshold is not None:
                    threshold_str = f"{threshold}ms"
                    utilization = (duration / threshold * 100) if threshold > 0 else 0
                    util_str = f"{utilization:.1f}%"
                    bar = make_timing_bar(duration, threshold)
                    status = "✅" if duration < threshold else "❌"
                else:
                    threshold_str = "-"
                    util_str = "-"
                    bar = make_timing_bar(duration, 1000)
                    status = "✅"

                lines.append(
                    f"| {short_name} | {format_duration(duration)} | {per_call_str} | {threshold_str} | {bar} {util_str} | {status} |"
                )

            lines.append("")

            # Extra details: iterations and call counts
            has_extra = any(
                p.get("iterations") or p.get("total_calls") or p.get("columns")
                for p in perf_details
            )
            if has_extra:
                lines.append("<details>")
                lines.append("<summary>📊 Benchmark parameters</summary>")
                lines.append("")
                lines.append("| Test | Iterations | Calls/Columns |")
                lines.append("|------|------------|---------------|")
                for p in perf_details:
                    short_name = p["name"].replace("sql::generator_test::", "")
                    iters = p.get("iterations", "-")
                    calls = p.get("total_calls") or p.get("columns") or p.get("types", "-")
                    lines.append(f"| {short_name} | {iters} | {calls} |")
                lines.append("")
                lines.append("</details>")
                lines.append("")

        else:
            # No [PERF] output parsed, show basic list
            lines.append("| Test | Duration | Threshold | Status |")
            lines.append("|------|----------|-----------|--------|")
            for t in perf_tests:
                name = t["name"]
                short_name = name.replace("sql::generator_test::", "")
                duration = json_timing.get(name, 0)
                threshold = get_perf_threshold(name)
                if threshold is not None:
                    threshold_str = f"{threshold}ms"
                    status = "✅" if duration < threshold else "❌"
                else:
                    threshold_str = "-"
                    status = "✅"
                duration_str = format_duration(duration) if duration > 0 else "-"
                lines.append(f"| {short_name} | {duration_str} | {threshold_str} | {status} |")
            lines.append("")

    # All tests timing summary (from JSON timing)
    if json_timing:
        lines.append("### ⏱️ All Tests Timing")
        lines.append("")

        # Sort by duration descending
        sorted_timing = sorted(json_timing.items(), key=lambda x: x[1], reverse=True)
        max_duration = max(json_timing.values()) if json_timing else 1

        lines.append("| Test | Duration | Bar |")
        lines.append("|------|----------|-----|")

        for name, duration_ms in sorted_timing:
            short_name = name.replace("sql::generator_test::", "")
            bar = make_timing_bar(duration_ms, max_duration, max_bar_width=30)
            lines.append(f"| {short_name} | {format_duration(duration_ms)} | {bar} |")

        lines.append("")

    # Test list summary
    lines.append("### 📋 Test List")
    lines.append("")
    lines.append("<details>")
    lines.append("<summary>Click to expand full test list</summary>")
    lines.append("")
    for t in passed:
        icon = "⚡" if "perf" in t["name"].lower() else "✅"
        short_name = t["name"].replace("sql::generator_test::", "")
        timing_str = ""
        if t["name"] in json_timing:
            timing_str = f" ({format_duration(json_timing[t['name']])})"
        lines.append(f"- {icon} {short_name}{timing_str}")
    for t in failed:
        short_name = t["name"].replace("sql::generator_test::", "")
        lines.append(f"- ❌ {short_name}")
    lines.append("")
    lines.append("</details>")
    lines.append("")

    return "\n".join(lines)


def main():
    json_file = "test-results.json"
    text_file = "test-results.txt"

    passed, failed, ignored, json_timing = parse_json_results(json_file)
    summary, text_passed, text_failed, text_ignored, perf_details = parse_text_results(text_file)

    # Fallback: if JSON parsing yielded no results, use text-parsed individual tests
    if not passed and not failed:
        passed = text_passed
        failed = text_failed
        ignored = text_ignored

    categories = categorize_tests(passed, failed)

    report = generate_report(passed, failed, ignored, summary, perf_details, json_timing, categories)

    # Write markdown report
    with open("test-report.md", "w") as f:
        f.write(report)

    # Write structured JSON report
    report_data = {
        "total": len(passed) + len(failed),
        "passed": len(passed),
        "failed": len(failed),
        "ignored": len(ignored),
        "pass_rate": (len(passed) / (len(passed) + len(failed)) * 100)
        if (len(passed) + len(failed)) > 0
        else 0,
        "stability_tests": categories["stability"],
        "performance_tests": categories["performance"],
        "performance_details": perf_details,
        "all_timing_ms": json_timing,
    }
    with open("test-report.json", "w") as f:
        json.dump(report_data, f, indent=2)

    print(report)


if __name__ == "__main__":
    main()
