..
   # *******************************************************************************
   # Copyright (c) 2026 Contributors to the Eclipse Foundation
   #
   # See the NOTICE file(s) distributed with this work for additional
   # information regarding copyright ownership.
   #
   # This program and the accompanying materials are made available under the
   # terms of the Apache License Version 2.0 which is available at
   # https://www.apache.org/licenses/LICENSE-2.0
   #
   # SPDX-License-Identifier: Apache-2.0
   # *******************************************************************************

.. document:: Datarouter Implementation Inspection Checklist
  :id: doc__datarouter_impl_inspection
  :status: draft
  :version: 1
  :safety: QM
  :security: YES
  :realizes: wp__sw_implementation_inspection

Implementation Inspection Checklist
===================================

Purpose
-------

The purpose of this checklist is to collect the topics to be checked during implementation,
i.e. in the detailed design and the source code of the units.

The checklist shall be agnostic to which programming language is used. Differences shall be treated
by linking to C++ or Rust specific documentation.

Conduct
-------

As described in the concept :need:`doc_concept__wp_inspections` the following "inspection roles" are expected to be filled:

- content responsible (author): <contributor/committer explicitly named here, who is the main author, as can be seen in config mgt tooling>
- reviewer: <contributor/committer explicitly named here, who is the main content reviewer, must be different from content responsible>
- moderator: <committer explicitly named here, who is is the safety manager, security manager or quality manager initiating the inspection>

Checklist
---------

It is mandatory to fill in the "passed" column with "yes" or "no" for each checklist item and additionally to add in the remarks why it is passed or not passed.
In case of "no" an issue link to the issue tracking system has to be added in the last column (if not solved in the same issue).
See also :need:`doc_concept__wp_inspections` for further information about reviews in general and inspection in particular.

Scope: this checklist covers the Datarouter daemon (:need:`comp__datarouter`, ``score/datarouter``).
The parts of ``logging_architecture.md`` about ``mw::log`` are kept as
context and were not compared with the code.

.. list-table:: Implementation Checklist
   :header-rows: 1
   :widths: 10,25,30,6,35,8

   * - Review ID
     - Acceptance Criteria
     - Guidance
     - Passed
     - Remarks
     - Issue link
   * - IMPL_01_01
     - Is the design according to guidelines?
     - see :need:`gd_temp__detailed_design` and :need:`doc_concept__imp_concept`
       (e.g. are the views done with the proposed UML diagrams)
     - yes
     - What was checked:

       - The detailed design follows the template (description, decisions,
         constraints, external dependencies, units, interfaces, static and dynamic
         views).
       - The views are UML diagrams, and the PlantUML diagrams pass the
         ``architectural_design`` target
         ``//score/datarouter/doc/design:datarouter_architectural_design``.
       - The folder structure follows the module template
         (``docs/components/datarouter/detailed_design``).
       - The design files stay in ``score/datarouter/doc/design`` and are linked
         into this folder by symbolic links, so the exports from the internal
         repository are not affected.

       What was fixed:

       - Nine diagrams of ``logging_architecture.md`` were loaded from the internet
         by URL. They are local files now.
       - Three diagrams that no document used are now shown in the design. A
         unit diagram that the validator needs was added.
     -
   * - IMPL_01_02
     - Is the implementation according to specification?
     - Check if the linked component requirements are fulfilled
       and detailed design also matches architecture description.
     - no
     - What was checked:

       - The detailed design against the component architecture. Both
         architecture interfaces of the component
         (``logic_arc_int__logging__shm`` and
         ``logic_arc_int__logging__session_ctrl_channel``) are described and
         mapped to the code.
       - That each of the six component requirements has a test linked to it.
       - The requirement texts against the code.
       - The code and the design for behaviour that no requirement asks for.

       What was fixed:

       - ``comp_req__data_router__dlt_quota_bw_reduction`` had no test linked.
         Four existing tests are linked to it now.
       - ``dlt_server``, ``dlt_multiple_channels``, ``dlt_message_filtering`` and
         ``dlt_bw_quota_config`` had existing tests that were not linked. They
         are linked now. Only ``RecordProperty`` lines were added, no test logic
         was changed.

       Open Points:

       - Nothing tests that messages are really dropped when a source sends too
         much. The tests for ``comp_req__data_router__dlt_quota_bw_reduction``
         only check that the quota switch can be configured. The code is in
         ``DataRouter::SourceSession`` and this class has no unit test. Action:
         add a test. Gaps in the other requirements are listed in
         `ticket #319 <https://github.com/eclipse-score/logging/issues/319>`_.
       - The requirements disagree on where ``log-channels.json`` is:

         - message filtering: ``<data_router binary location>/../etc/log-channels.json``
         - multiple channels: ``<data_router binary location>/./etc/log-channels.json``
         - quota: ``./etc/log-channels.json`` relative to ``/opt/``

         The code reads ``./etc/log-channels.json`` relative to the working
         directory, or the path given with the option ``-c``. Action: make the
         three requirement texts say the same, and match the code.
       - Some behaviour has no component requirement: the sessions with the
         clients (connect, watchdog, early disconnect), reading the shared memory,
         the statistics, the file transfer, the configuration sessions and the
         persistent configuration. Action: decide which of these need a
         requirement.
       - The goal "implement traffic shaping to prevent DLT data flooding" in
         ``logging_architecture.md`` is only met by the per-source quota, which
         drops messages. There is no shaping or delay. Action: confirm that the
         quota is the intended solution.
     - Requirement tests: `#319 <https://github.com/eclipse-score/logging/issues/319>`_.
       Other points: to be added.
   * - IMPL_01_03
     - Are the design decisions and constraints documented?
     - Check also for plausibility of these.
     - yes
     - What was checked:

       - Every decision and constraint in the detailed design against the code:
         acquire watchdog (1000 ms, 3 misses), 100 ms tick, quota per statistics
         period, order of the filters, mandatory ``log-channels.json``, build
         flags and the stub persistent dictionary.
       - Plausibility: the values fit a design with one worker thread for all
         sessions and non-blocking notifications.

       What was found (the texts were not changed):

       - ``shm_apis.md`` says that a POSIX mutex in the shared memory protects the
         buffer and that the Datarouter modifies the shared memory. The code does
         neither: writers use wait-free alternating buffers and the Datarouter
         maps the memory read-only. Action: correct the text.
       - Decisions and constraints that are only visible in the code are in no
         document and in no diagram: watchdog values (1000 ms, 3 misses), 100 ms
         tick, 10 s statistics period, order of the filters, build flags, paths
         of the configuration files and what happens if one is missing,
         ``ENOBUFS`` handling, untrusted input from clients. The detailed design
         refers to the diagrams only. Action: show them in the diagrams.
       - No diagram shows the bandwidth quota per source. Action: add it to a
         diagram.

       To confirm in the review (not a failure of this item):

       - The quota is measured over the current statistics period (10 s). A source
         is dropped only after its rate in that period is above the quota, so a
         short burst can get through. The requirement
         ``comp_req__data_router__dlt_quota_bw_reduction`` does not say how fast
         the reduction must react. Action: confirm that this is intended.
     -
   * - IMPL_01_04
     - Are all external libraries used by the component specified in the detailed design?
     - Check the automated dependency analysis.
       Also make sure ASIL rated units also only use ASIL rated libraries.
     - no
     - What was checked:

       - The direct dependencies of all non-test targets were collected with
         ``bazel query 'labels(deps, kind("cc_library|cc_binary", //score/datarouter/...) except attr(testonly, 1, //score/datarouter/...)) except //score/datarouter/...'``
         and compared with the ``#include`` lines in the sources.
       - The Datarouter is not safety rated (QM). The rule "ASIL units only use
         ASIL-rated libraries" does not apply to it. The ratings of the
         dependencies come from the ``comp`` needs of the providing components.

       Open Points:

       - ``//score/mw/log:log`` is deprecated (removed in v0.8). 9 non-test
         targets use it. Action: use ``@score_baselibs//score/mw/log``.
       - ``high_resolution_steady_clock`` (``score_baselibs``) is deprecated and
         used by ``DataRouter::SourceSession``. Action: move to
         ``HighPrecisionLocalSteadyClock``.
       - ``string_literal`` and ``static_reflection_with_serialization/visitor``
         are declared as dependencies, but no source file includes them.
         Action: check and remove.
       - The mock libraries (``*_mock`` in ``BUILD``) are not ``testonly`` and
         depend on ``gtest_main``. Action: mark them ``testonly``.
       - ``visibility`` lists in ``BUILD``, ``dlt_filetransfer_trigger_lib/BUILD``
         and ``test/ut/etc/BUILD`` name packages that do not exist here
         (``//platform/aas/...``, ``//pqr/...``). Action: remove them or move them
         to the integration repository.
       - The safety rating of ``score_communication`` ``message_passing``,
         ``rapidjson`` and ``score_baselibs`` ``result`` and ``string_manipulation``
         is not stated in the component documentation. Action: get the ratings
         confirmed.
     - To be added
   * - IMPL_02_01
     - Are the static and dynamic code analysis reports verified for violations?
     - All violations in ASIL related code must be justified. This includes the checks of coding guidelines.
     - no
     - What was checked (host run, Linux x86_64):

       - Unit tests: 21 of 21 passed. The QNX tests were not run.
       - clang-tidy, ASan/UBSan/LSan and TSan: no findings.
       - Memcheck (valgrind): no errors and no definite leaks. Googletest prints
         padding bytes of mock values and valgrind reports them as false errors.
         They are suppressed and are not in the code under test.
       - Compiler warnings are errors in the build. It passes with gcc and clang.
       - Line and branch coverage: 82.6 % and 76.1 % (target 100 %). The details
         per file, causes and actions are in
         `ticket #318 <https://github.com/eclipse-score/logging/issues/318>`_.
         The host numbers are a proxy, the reporting platform is QNX.
       - The static-analysis suppressions (``coverity[...]``, ``NOLINT``) were
         checked for a justification next to them (see ticket #318).

       Open Points:

       - Coverity is not done (no licence). Action: run it or provide the
         results.
       - The QNX test and coverage results are not attached. Action: attach them.
       - ``DataRouter`` and ``SourceSession`` (``data_router.cpp``) have no unit
         test. Action: add tests (see ticket #318).
       - ``test_logger.cpp`` and ``tracingTest.cpp`` are in the test folder but
         in no Bazel target. They test ``mw::log``. Action: move them to
         ``mw::log`` or add targets.
       - ``src/daemon/priority_boost.cpp`` is in no Bazel target and includes a
         header that does not exist. ``ReadPersistentLoggingConfig`` is built and
         tested, but no production code calls it. Action: remove the dead code or
         add the missing parts.
       - 9 of 159 suppressions have no justification next to them. Action: add
         one (list in ticket #318).
       - 94 suppressions use AUTOSAR C++14 rule ids. The project guideline is
         MISRA C++:2023. Action: map them to MISRA C++:2023 and check that each
         has a justification.
     - Coverage: `#318 <https://github.com/eclipse-score/logging/issues/318>`_.
       Coverity and the other points: to be added.
   * - IMPL_02_02
     - Do manual checks, that are derived from the coding guideline, find no safety critical error?
     - Check this for the programming language used (e.g. C++ <link_to_checks_list>, Rust <link_to_checks_list>)
     - no
     - What was checked:

       - The checks list was identified: the
         `C++ guideline <https://eclipse-score.github.io/score/main/contribute/development/cpp/coding_guidelines.html>`_
         (MISRA C++:2023) with the
         `code analysis <https://eclipse-score.github.io/score/main/contribute/development/cpp/code_analysis.html>`_
         page. The Datarouter has no Rust code.

       Open Points:

       - The manual checks are not done yet. Action: run them and update this
         remark.
       - The guideline asks to avoid the prefix ``I`` for interfaces. These
         interfaces use it: ``IPersistentDictionary``, ``ISession``,
         ``ILogParser``, ``ILogParserFactory``, ``IDiagnosticJobHandler`` and
         ``IDltLogServer``. Action: decide whether this is an accepted deviation
         or the names are changed.
     - To be added
   * - IMPL_03_01
     - Do the UID of the interface in component documentation match the implemented interface names of the unit?
     - Compare interface UIDs (which contains the interface name) in component architecture/detailed design documentation
       with public interfaces in source code (e.g. API headers, traits, public types/functions).
     - no
     - What was checked:

       - The architecture interface UIDs against the names in the code. The
         detailed design maps each UID to its source code interface.

       Open Points:

       - The names in the code do not match the UIDs. The mapping exists only in
         the detailed design. Examples:

         - ``logic_arc_int__logging__shm`` is implemented by
           ``ISharedMemoryReader`` (in ``score/mw/log``).
         - ``logic_arc_int__logging__session_ctrl_channel`` is implemented by
           ``MessagePassingServer``, ``ISession`` and ``ISessionHandle``.
         - The DLT output (UDP multicast) and the configuration socket
           (``UnixDomainServer``) are interfaces of the Datarouter, but have no
           UID in the architecture.

         Action: decide whether to change the UIDs in the architecture, rename
         the interfaces in the code, or accept the mapping in the detailed
         design as a documented deviation.
     - To be added
   * - IMPL_03_02
     - Are detailed design and source code consistent and is the respective traceability established ?
     - Check if available static and dynamic design diagrams and the textual descriptions match the code
       (e.g. naming of interfaces, units, functions/operations/messages, data types).
       Check if the folder/file names of the units and its source code matches the intended functionality.
       For example if a unit is named "communication" it should not contain code for "data processing".
     - no
     - What was checked:

       - The Datarouter texts of ``logging_architecture.md`` and ``shm_apis.md``,
         the README and the units table against the code.
       - The diagrams against the code. The session, watchdog, early disconnect
         and configuration session diagrams matched.

       Findings in the texts and diagrams (not changed):

       - The file transfer class diagram
         (``file_transfer_feature_class_diagram.puml``) shows the handler
         factories (``FileTransferStreamHandlerFactory``,
         ``StubFileTransferHandlerFactory``) and the ``Output`` classes. Only
         tests use the factories. In the code ``DltLogServer`` owns the handler
         (``fthandler_``). Action: rewrite the diagram after the code.
       - ``package-datarouter.puml`` lists classes that do not exist
         (``AraPerPersistentDictionary``, ``SysedrHandler`` and others). Action:
         remove them.
       - The filtering diagram (``dlt_message_filtering_backend.puml``) is
         incomplete and names the wrong file for the defaults. Action: rewrite
         it after ``DltLogServer::FilterAndCall``.
       - ``shm_apis.md`` describes a mutex that does not exist (see IMPL_01_03).
       - ``logging_architecture.md`` does not describe the watchdog values, the
         configuration sessions, the loop timing, the build flags and the paths
         of the configuration files (see IMPL_01_03).

       Open Points:

       - The quota text in ``logging_architecture.md`` says that after the first
         violation all further records of the source are dropped and that the
         flag is not reset. The code clears the flag with the statistics report every 10 s
         (``DataRouter::SourceSession``, ``data_router.cpp``), so the source is
         enforced again only if it exceeds the quota in the next period. Action:
         decide whether the code or the text is intended, then change that one.
       - Four persistent logging diagrams (``persistent_logging_*``,
         ``persistent-cleanup-*``) describe classes that are not in this
         repository (for example ``SysedrHandler``, ``ZlibCompressedFile``). The
         persistent dictionary here is a stub and no document uses the diagrams.
         Action: remove them or move them to the repository with the
         implementation.
       - ``ipc_status_quo.puml``, ``context-details.puml``,
         ``seq-tracing-startup.puml`` and ``remote_logging.plantuml`` are not used
         in any document and were not compared with the code. Action: compare
         and use them, or remove them.
       - The parts of ``logging_architecture.md`` about ``mw::log`` (including
         ``seq-trace.puml`` and ``dlt_message_filtering_frontend.puml``) are
         context only and were not compared with the code. They belong to the
         ``mw::log`` frontend (eclipse-score/logging#327).
       - The README shows a statistics message without the fields ``rate``,
         ``quota rate``, ``quota enforcement`` and ``IPC count``. Action: update
         the example.
       - The default path of ``class-id.json`` in ``socketserver.h``
         (``/bmw/platform/opt/datarouter/etc/class-id.json``) is a path of the
         internal platform. Action: decide the default path for this repository.
       - The Bazel flag ``enable_thin_network_sender`` is defined, but no target
         uses it. Action: remove it or document its purpose.
       - The folders ``src/daemon`` and ``datarouter`` hold several
         responsibilities (message passing, DLT server, channels, socket server).
         Action: check whether the folder names should follow the units of the
         detailed design.
     - To be added
