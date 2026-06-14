# Copilot 지침 (한국어 우선)

모든 응답 및 주석: 한글 우선
- 이 리포지토리에서 Copilot 또는 자동화된 어시스턴트의 모든 응답과 코드 주석은 한국어(한글)를 우선 사용하십시오. 필요한 경우 영어 보조 문장을 병기할 수 있습니다.

목적
- Copilot CLI 세션과 자동화 도우미를 위한 리포지토리 특화 간단 지침 제공.

리포지토리 스냅샷
- 현재 리포지토리는 최상위 README.md(제목만 포함)만 존재합니다. 작성 시점에 Cargo.toml, src/, CI 워크플로우는 감지되지 않았습니다.

1) 빌드, 테스트, 린트 명령 (Rust — Cargo.toml이 있을 때)
- 빌드: cargo build
- 실행(바이너리): cargo run --bin <name> 또는 cargo run
- 전체 테스트: cargo test
- 단일 테스트 실행(이름): cargo test <test_name>
- 정확히 단일 테스트 실행(테스트 러너에 직접 전달): cargo test <test_name> -- --exact
- 통합 테스트 파일 실행: cargo test --test <file_name>
- 포맷/포맷 검사: cargo fmt  (검사만: cargo fmt -- --check)
- 린트: cargo clippy --all-targets --all-features -- -D warnings
- 워크스페이스/특정 크레이트 지정: cargo build -p <crate-name>, cargo test -p <crate-name>

(비-Rust 매니페스트가 추가되면 해당 툴체인의 네이티브 명령을 우선 사용하세요.)

2) 고수준 아키텍처 (리포지토리 전체를 파악하는 방법)
- 크레이트와 워크스페이스 감지: 루트 또는 서브폴더에서 Cargo.toml을 찾으세요. 멤버 매핑은 `cargo metadata --no-deps --format-version 1` 사용.
- 전형적 레이아웃:
  - Cargo.toml (루트 또는 각 크레이트)
  - src/lib.rs (라이브러리) 및/또는 src/main.rs (바이너리)
  - tests/ (통합 테스트), benches/, examples/
  - workspace 멤버는 하위 폴더에 위치
- 멀티 크레이트 리포지토리에서는 특정 크레이트에 수정이 제한된 경우 `cargo test -p <crate>` 및 `cargo clippy -p <crate>`를 권장.
- 크로스-크레이트 심볼 추적에는 ripgrep(rg)을 사용해 호출 위치와 공개 API 영향 범위를 확인하세요.

3) 이 리포지토리에서의 주요 규약
- 변경은 정밀하게: 관련 없는 파일/크레이트 변경은 피하세요.
- 워크스페이스 경계 존중: Cargo.toml의 workspace 멤버 수정은 요청이 있을 때만 수행하세요.
- 포맷/린트: 수정 후 및 PR 전 `cargo fmt`와 `cargo clippy` 실행. 로컬에서는 `-D warnings`를 권장해 문제를 조기에 발견하세요.
- 테스트: 단위 테스트는 모듈 내부(src/ 내 테스트)로, 크로스-크레이트 동작은 tests/에 통합 테스트로 추가하세요.
- 빠른 반복: 실패한 단일 테스트를 고치려면 `cargo test <test_name>`을 사용하세요.

4) 전체 관점에서 먼저 확인할 파일
- Cargo.toml 및 워크스페이스 루트 파일
- src/lib.rs, src/main.rs (진입점)
- tests/ (통합 테스트 기대치)
- .github/workflows/* (CI 규격 — 현재 없음)

통합된 기존 문서
- README.md는 현재 제목만 포함하며 본 지침 생성 시 참고했습니다.

향후 세션 참고사항
- CI/워크플로우를 추가하거나 워크스페이스로 전환하면, 실제 명령과 CI 작업 이름(워크플로우 파일명 등)을 이 문서에 업데이트하여 Copilot이 목표 작업을 직접 실행하도록 하세요.

확장 요청
- 워크스페이스 감지 스크립트, CI 매핑, PR 체크리스트(포맷/테스트/린트) 등 추가를 원하면 어느 영역을 확장할지 알려주세요.
